//! A simplified AST that only contains items.

mod lower;

use std::{
    any::type_name,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Index, Range},
    sync::Arc,
};

use ast::{AstNode, NameOwner, StructKind};
use base_db::CrateId;
use either::Either;
use hir_expand::{
    ast_id_map::FileAstId,
    hygiene::Hygiene,
    name::{name, AsName, Name},
    HirFileId, InFile,
};
use la_arena::{Arena, Idx, RawIdx};
use profile::Count;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use syntax::{ast, match_ast, SyntaxKind};

use crate::{
    attr::{Attrs, RawAttrs},
    db::DefDatabase,
    generics::GenericParams,
    intern::Interned,
    path::{path, AssociatedTypeBinding, GenericArgs, ImportAlias, ModPath, Path, PathKind},
    type_ref::{Mutability, TraitRef, TypeBound, TypeRef},
    visibility::RawVisibility,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RawVisibilityId(u32);

impl RawVisibilityId {
    pub const PUB: Self = RawVisibilityId(u32::max_value());
    pub const PRIV: Self = RawVisibilityId(u32::max_value() - 1);
    pub const PUB_CRATE: Self = RawVisibilityId(u32::max_value() - 2);
}

impl fmt::Debug for RawVisibilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("RawVisibilityId");
        match *self {
            Self::PUB => f.field(&"pub"),
            Self::PRIV => f.field(&"pub(self)"),
            Self::PUB_CRATE => f.field(&"pub(crate)"),
            _ => f.field(&self.0),
        };
        f.finish()
    }
}

/// The item tree of a source file.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    _c: Count<Self>,

    top_level: SmallVec<[ModItem; 1]>,
    attrs: FxHashMap<AttrOwner, RawAttrs>,

    data: Option<Box<ItemTreeData>>,
}

impl ItemTree {
    pub(crate) fn file_item_tree_query(db: &dyn DefDatabase, file_id: HirFileId) -> Arc<ItemTree> {
        let _p = profile::span("item_tree_query").detail(|| format!("{:?}", file_id));
        let syntax = if let Some(node) = db.parse_or_expand(file_id) {
            if node.kind() == SyntaxKind::ERROR {
                // FIXME: not 100% sure why these crop up, but return an empty tree to avoid a panic
                return Default::default();
            }
            node
        } else {
            return Default::default();
        };

        let hygiene = Hygiene::new(db.upcast(), file_id);
        let ctx = lower::Ctx::new(db, hygiene.clone(), file_id);
        let mut top_attrs = None;
        let mut item_tree = match_ast! {
            match syntax {
                ast::SourceFile(file) => {
                    top_attrs = Some(RawAttrs::new(&file, &hygiene));
                    ctx.lower_module_items(&file)
                },
                ast::MacroItems(items) => {
                    ctx.lower_module_items(&items)
                },
                ast::MacroStmts(stmts) => {
                    // The produced statements can include items, which should be added as top-level
                    // items.
                    ctx.lower_macro_stmts(stmts)
                },
                ast::Pat(_pat) => {
                    // FIXME: This occurs because macros in pattern position are treated as inner
                    // items and expanded during block DefMap computation
                    return Default::default();
                },
                ast::Type(ty) => {
                    // Types can contain inner items. We return an empty item tree in this case, but
                    // still need to collect inner items.
                    ctx.lower_inner_items(ty.syntax())
                },
                ast::Expr(e) => {
                    // Macros can expand to expressions. We return an empty item tree in this case, but
                    // still need to collect inner items.
                    ctx.lower_inner_items(e.syntax())
                },
                _ => {
                    panic!("cannot create item tree from {:?} {}", syntax, syntax);
                },
            }
        };

        if let Some(attrs) = top_attrs {
            item_tree.attrs.insert(AttrOwner::TopLevel, attrs);
        }
        item_tree.shrink_to_fit();
        Arc::new(item_tree)
    }

    fn shrink_to_fit(&mut self) {
        if let Some(data) = &mut self.data {
            let ItemTreeData {
                imports,
                extern_crates,
                functions,
                params,
                structs,
                fields,
                unions,
                enums,
                variants,
                consts,
                statics,
                traits,
                impls,
                type_aliases,
                mods,
                macro_calls,
                macro_rules,
                macro_defs,
                vis,
                inner_items,
            } = &mut **data;

            imports.shrink_to_fit();
            extern_crates.shrink_to_fit();
            functions.shrink_to_fit();
            params.shrink_to_fit();
            structs.shrink_to_fit();
            fields.shrink_to_fit();
            unions.shrink_to_fit();
            enums.shrink_to_fit();
            variants.shrink_to_fit();
            consts.shrink_to_fit();
            statics.shrink_to_fit();
            traits.shrink_to_fit();
            impls.shrink_to_fit();
            type_aliases.shrink_to_fit();
            mods.shrink_to_fit();
            macro_calls.shrink_to_fit();
            macro_rules.shrink_to_fit();
            macro_defs.shrink_to_fit();

            vis.arena.shrink_to_fit();

            inner_items.shrink_to_fit();
        }
    }

    /// Returns an iterator over all items located at the top level of the `HirFileId` this
    /// `ItemTree` was created from.
    pub fn top_level_items(&self) -> &[ModItem] {
        &self.top_level
    }

    /// Returns the inner attributes of the source file.
    pub fn top_level_attrs(&self, db: &dyn DefDatabase, krate: CrateId) -> Attrs {
        self.attrs.get(&AttrOwner::TopLevel).unwrap_or(&RawAttrs::EMPTY).clone().filter(db, krate)
    }

    pub(crate) fn raw_attrs(&self, of: AttrOwner) -> &RawAttrs {
        self.attrs.get(&of).unwrap_or(&RawAttrs::EMPTY)
    }

    pub fn attrs(&self, db: &dyn DefDatabase, krate: CrateId, of: AttrOwner) -> Attrs {
        self.raw_attrs(of).clone().filter(db, krate)
    }

    pub fn inner_items_of_block(&self, block: FileAstId<ast::BlockExpr>) -> &[ModItem] {
        match &self.data {
            Some(data) => data.inner_items.get(&block).map(|it| &**it).unwrap_or(&[]),
            None => &[],
        }
    }

    fn data(&self) -> &ItemTreeData {
        self.data.as_ref().expect("attempted to access data of empty ItemTree")
    }

    fn data_mut(&mut self) -> &mut ItemTreeData {
        self.data.get_or_insert_with(Box::default)
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
struct ItemVisibilities {
    arena: Arena<RawVisibility>,
}

impl ItemVisibilities {
    fn alloc(&mut self, vis: RawVisibility) -> RawVisibilityId {
        match &vis {
            RawVisibility::Public => RawVisibilityId::PUB,
            RawVisibility::Module(path) if path.segments().is_empty() => match &path.kind {
                PathKind::Super(0) => RawVisibilityId::PRIV,
                PathKind::Crate => RawVisibilityId::PUB_CRATE,
                _ => RawVisibilityId(self.arena.alloc(vis).into_raw().into()),
            },
            _ => RawVisibilityId(self.arena.alloc(vis).into_raw().into()),
        }
    }
}

static VIS_PUB: RawVisibility = RawVisibility::Public;
static VIS_PRIV: RawVisibility = RawVisibility::Module(ModPath::from_kind(PathKind::Super(0)));
static VIS_PUB_CRATE: RawVisibility = RawVisibility::Module(ModPath::from_kind(PathKind::Crate));

#[derive(Default, Debug, Eq, PartialEq)]
struct ItemTreeData {
    imports: Arena<Import>,
    extern_crates: Arena<ExternCrate>,
    functions: Arena<Function>,
    params: Arena<Param>,
    structs: Arena<Struct>,
    fields: Arena<Field>,
    unions: Arena<Union>,
    enums: Arena<Enum>,
    variants: Arena<Variant>,
    consts: Arena<Const>,
    statics: Arena<Static>,
    traits: Arena<Trait>,
    impls: Arena<Impl>,
    type_aliases: Arena<TypeAlias>,
    mods: Arena<Mod>,
    macro_calls: Arena<MacroCall>,
    macro_rules: Arena<MacroRules>,
    macro_defs: Arena<MacroDef>,

    vis: ItemVisibilities,

    inner_items: FxHashMap<FileAstId<ast::BlockExpr>, SmallVec<[ModItem; 1]>>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AttrOwner {
    /// Attributes on an item.
    ModItem(ModItem),
    /// Inner attributes of the source file.
    TopLevel,

    Variant(Idx<Variant>),
    Field(Idx<Field>),
    Param(Idx<Param>),
}

macro_rules! from_attrs {
    ( $( $var:ident($t:ty) ),+ ) => {
        $(
            impl From<$t> for AttrOwner {
                fn from(t: $t) -> AttrOwner {
                    AttrOwner::$var(t)
                }
            }
        )+
    };
}

from_attrs!(ModItem(ModItem), Variant(Idx<Variant>), Field(Idx<Field>), Param(Idx<Param>));

/// Trait implemented by all item nodes in the item tree.
pub trait ItemTreeNode: Clone {
    type Source: AstNode + Into<ast::Item>;

    fn ast_id(&self) -> FileAstId<Self::Source>;

    /// Looks up an instance of `Self` in an item tree.
    fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self;

    /// Downcasts a `ModItem` to a `FileItemTreeId` specific to this type.
    fn id_from_mod_item(mod_item: ModItem) -> Option<FileItemTreeId<Self>>;

    /// Upcasts a `FileItemTreeId` to a generic `ModItem`.
    fn id_to_mod_item(id: FileItemTreeId<Self>) -> ModItem;
}

pub struct FileItemTreeId<N: ItemTreeNode> {
    index: Idx<N>,
    _p: PhantomData<N>,
}

impl<N: ItemTreeNode> Clone for FileItemTreeId<N> {
    fn clone(&self) -> Self {
        Self { index: self.index, _p: PhantomData }
    }
}
impl<N: ItemTreeNode> Copy for FileItemTreeId<N> {}

impl<N: ItemTreeNode> PartialEq for FileItemTreeId<N> {
    fn eq(&self, other: &FileItemTreeId<N>) -> bool {
        self.index == other.index
    }
}
impl<N: ItemTreeNode> Eq for FileItemTreeId<N> {}

impl<N: ItemTreeNode> Hash for FileItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<N: ItemTreeNode> fmt::Debug for FileItemTreeId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.index.fmt(f)
    }
}

#[derive(Debug)]
pub struct ItemTreeId<N: ItemTreeNode> {
    file: HirFileId,
    pub value: FileItemTreeId<N>,
}

impl<N: ItemTreeNode> ItemTreeId<N> {
    pub fn new(file: HirFileId, idx: FileItemTreeId<N>) -> Self {
        Self { file, value: idx }
    }

    pub fn file_id(self) -> HirFileId {
        self.file
    }

    pub fn item_tree(self, db: &dyn DefDatabase) -> Arc<ItemTree> {
        db.file_item_tree(self.file)
    }
}

impl<N: ItemTreeNode> Copy for ItemTreeId<N> {}
impl<N: ItemTreeNode> Clone for ItemTreeId<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: ItemTreeNode> PartialEq for ItemTreeId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file && self.value == other.value
    }
}

impl<N: ItemTreeNode> Eq for ItemTreeId<N> {}

impl<N: ItemTreeNode> Hash for ItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.value.hash(state);
    }
}

macro_rules! mod_items {
    ( $( $typ:ident in $fld:ident -> $ast:ty ),+ $(,)? ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum ModItem {
            $(
                $typ(FileItemTreeId<$typ>),
            )+
        }

        $(
            impl From<FileItemTreeId<$typ>> for ModItem {
                fn from(id: FileItemTreeId<$typ>) -> ModItem {
                    ModItem::$typ(id)
                }
            }
        )+

        $(
            impl ItemTreeNode for $typ {
                type Source = $ast;

                fn ast_id(&self) -> FileAstId<Self::Source> {
                    self.ast_id
                }

                fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self {
                    &tree.data().$fld[index]
                }

                fn id_from_mod_item(mod_item: ModItem) -> Option<FileItemTreeId<Self>> {
                    if let ModItem::$typ(id) = mod_item {
                        Some(id)
                    } else {
                        None
                    }
                }

                fn id_to_mod_item(id: FileItemTreeId<Self>) -> ModItem {
                    ModItem::$typ(id)
                }
            }

            impl Index<Idx<$typ>> for ItemTree {
                type Output = $typ;

                fn index(&self, index: Idx<$typ>) -> &Self::Output {
                    &self.data().$fld[index]
                }
            }
        )+
    };
}

mod_items! {
    Import in imports -> ast::Use,
    ExternCrate in extern_crates -> ast::ExternCrate,
    Function in functions -> ast::Fn,
    Struct in structs -> ast::Struct,
    Union in unions -> ast::Union,
    Enum in enums -> ast::Enum,
    Const in consts -> ast::Const,
    Static in statics -> ast::Static,
    Trait in traits -> ast::Trait,
    Impl in impls -> ast::Impl,
    TypeAlias in type_aliases -> ast::TypeAlias,
    Mod in mods -> ast::Module,
    MacroCall in macro_calls -> ast::MacroCall,
    MacroRules in macro_rules -> ast::MacroRules,
    MacroDef in macro_defs -> ast::MacroDef,
}

macro_rules! impl_index {
    ( $($fld:ident: $t:ty),+ $(,)? ) => {
        $(
            impl Index<Idx<$t>> for ItemTree {
                type Output = $t;

                fn index(&self, index: Idx<$t>) -> &Self::Output {
                    &self.data().$fld[index]
                }
            }
        )+
    };
}

impl_index!(fields: Field, variants: Variant, params: Param);

impl Index<RawVisibilityId> for ItemTree {
    type Output = RawVisibility;
    fn index(&self, index: RawVisibilityId) -> &Self::Output {
        match index {
            RawVisibilityId::PRIV => &VIS_PRIV,
            RawVisibilityId::PUB => &VIS_PUB,
            RawVisibilityId::PUB_CRATE => &VIS_PUB_CRATE,
            _ => &self.data().vis.arena[Idx::from_raw(index.0.into())],
        }
    }
}

impl<N: ItemTreeNode> Index<FileItemTreeId<N>> for ItemTree {
    type Output = N;
    fn index(&self, id: FileItemTreeId<N>) -> &N {
        N::lookup(self, id.index)
    }
}

/// A desugared `use` import.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Import {
    pub path: Interned<ModPath>,
    pub alias: Option<ImportAlias>,
    pub visibility: RawVisibilityId,
    pub is_glob: bool,
    /// AST ID of the `use` or `extern crate` item this import was derived from. Note that many
    /// `Import`s can map to the same `use` item.
    pub ast_id: FileAstId<ast::Use>,
    /// Index of this `Import` when the containing `Use` is visited via `ModPath::expand_use_item`.
    ///
    /// This can be used to get the `UseTree` this `Import` corresponds to and allows emitting
    /// precise diagnostics.
    pub index: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternCrate {
    pub name: Name,
    pub alias: Option<ImportAlias>,
    pub visibility: RawVisibilityId,
    pub ast_id: FileAstId<ast::ExternCrate>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub generic_params: Interned<GenericParams>,
    pub abi: Option<Interned<str>>,
    pub params: IdRange<Param>,
    pub ret_type: Interned<TypeRef>,
    pub ast_id: FileAstId<ast::Fn>,
    pub(crate) flags: FnFlags,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Param {
    Normal(Interned<TypeRef>),
    Varargs,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub(crate) struct FnFlags {
    pub(crate) bits: u8,
}
impl FnFlags {
    pub(crate) const HAS_SELF_PARAM: u8 = 1 << 0;
    pub(crate) const HAS_BODY: u8 = 1 << 1;
    pub(crate) const IS_DEFAULT: u8 = 1 << 2;
    pub(crate) const IS_CONST: u8 = 1 << 3;
    pub(crate) const IS_ASYNC: u8 = 1 << 4;
    pub(crate) const IS_UNSAFE: u8 = 1 << 5;
    /// Whether the function is located in an `extern` block (*not* whether it is an
    /// `extern "abi" fn`).
    pub(crate) const IS_IN_EXTERN_BLOCK: u8 = 1 << 6;
    pub(crate) const IS_VARARGS: u8 = 1 << 7;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub generic_params: Interned<GenericParams>,
    pub fields: Fields,
    pub ast_id: FileAstId<ast::Struct>,
    pub kind: StructDefKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructDefKind {
    /// `struct S { ... }` - type namespace only.
    Record,
    /// `struct S(...);`
    Tuple,
    /// `struct S;`
    Unit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Union {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub generic_params: Interned<GenericParams>,
    pub fields: Fields,
    pub ast_id: FileAstId<ast::Union>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Enum {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub generic_params: Interned<GenericParams>,
    pub variants: IdRange<Variant>,
    pub ast_id: FileAstId<ast::Enum>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Const {
    /// const _: () = ();
    pub name: Option<Name>,
    pub visibility: RawVisibilityId,
    pub type_ref: Interned<TypeRef>,
    pub ast_id: FileAstId<ast::Const>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Static {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub mutable: bool,
    /// Whether the static is in an `extern` block.
    pub is_extern: bool,
    pub type_ref: Interned<TypeRef>,
    pub ast_id: FileAstId<ast::Static>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trait {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub generic_params: Interned<GenericParams>,
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub bounds: Box<[TypeBound]>,
    pub items: Box<[AssocItem]>,
    pub ast_id: FileAstId<ast::Trait>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Impl {
    pub generic_params: Interned<GenericParams>,
    pub target_trait: Option<Interned<TraitRef>>,
    pub self_ty: Interned<TypeRef>,
    pub is_negative: bool,
    pub items: Box<[AssocItem]>,
    pub ast_id: FileAstId<ast::Impl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub name: Name,
    pub visibility: RawVisibilityId,
    /// Bounds on the type alias itself. Only valid in trait declarations, eg. `type Assoc: Copy;`.
    pub bounds: Box<[TypeBound]>,
    pub generic_params: Interned<GenericParams>,
    pub type_ref: Option<Interned<TypeRef>>,
    pub is_extern: bool,
    pub ast_id: FileAstId<ast::TypeAlias>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mod {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub kind: ModKind,
    pub ast_id: FileAstId<ast::Module>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ModKind {
    /// `mod m { ... }`
    Inline { items: Box<[ModItem]> },

    /// `mod m;`
    Outline {},
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MacroCall {
    /// Path to the called macro.
    pub path: Interned<ModPath>,
    pub ast_id: FileAstId<ast::MacroCall>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MacroRules {
    /// The name of the declared macro.
    pub name: Name,
    pub ast_id: FileAstId<ast::MacroRules>,
}

/// "Macros 2.0" macro definition.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MacroDef {
    pub name: Name,
    pub visibility: RawVisibilityId,
    pub ast_id: FileAstId<ast::MacroDef>,
}

macro_rules! impl_froms {
    ($e:ident { $($v:ident ($t:ty)),* $(,)? }) => {
        $(
            impl From<$t> for $e {
                fn from(it: $t) -> $e {
                    $e::$v(it)
                }
            }
        )*
    }
}

impl ModItem {
    pub fn as_assoc_item(&self) -> Option<AssocItem> {
        match self {
            ModItem::Import(_)
            | ModItem::ExternCrate(_)
            | ModItem::Struct(_)
            | ModItem::Union(_)
            | ModItem::Enum(_)
            | ModItem::Static(_)
            | ModItem::Trait(_)
            | ModItem::Impl(_)
            | ModItem::Mod(_)
            | ModItem::MacroRules(_)
            | ModItem::MacroDef(_) => None,
            ModItem::MacroCall(call) => Some(AssocItem::MacroCall(*call)),
            ModItem::Const(konst) => Some(AssocItem::Const(*konst)),
            ModItem::TypeAlias(alias) => Some(AssocItem::TypeAlias(*alias)),
            ModItem::Function(func) => Some(AssocItem::Function(*func)),
        }
    }

    pub fn downcast<N: ItemTreeNode>(self) -> Option<FileItemTreeId<N>> {
        N::id_from_mod_item(self)
    }

    pub fn ast_id(&self, tree: &ItemTree) -> FileAstId<ast::Item> {
        match self {
            ModItem::Import(it) => tree[it.index].ast_id().upcast(),
            ModItem::ExternCrate(it) => tree[it.index].ast_id().upcast(),
            ModItem::Function(it) => tree[it.index].ast_id().upcast(),
            ModItem::Struct(it) => tree[it.index].ast_id().upcast(),
            ModItem::Union(it) => tree[it.index].ast_id().upcast(),
            ModItem::Enum(it) => tree[it.index].ast_id().upcast(),
            ModItem::Const(it) => tree[it.index].ast_id().upcast(),
            ModItem::Static(it) => tree[it.index].ast_id().upcast(),
            ModItem::Trait(it) => tree[it.index].ast_id().upcast(),
            ModItem::Impl(it) => tree[it.index].ast_id().upcast(),
            ModItem::TypeAlias(it) => tree[it.index].ast_id().upcast(),
            ModItem::Mod(it) => tree[it.index].ast_id().upcast(),
            ModItem::MacroCall(it) => tree[it.index].ast_id().upcast(),
            ModItem::MacroRules(it) => tree[it.index].ast_id().upcast(),
            ModItem::MacroDef(it) => tree[it.index].ast_id().upcast(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AssocItem {
    Function(FileItemTreeId<Function>),
    TypeAlias(FileItemTreeId<TypeAlias>),
    Const(FileItemTreeId<Const>),
    MacroCall(FileItemTreeId<MacroCall>),
}

impl_froms!(AssocItem {
    Function(FileItemTreeId<Function>),
    TypeAlias(FileItemTreeId<TypeAlias>),
    Const(FileItemTreeId<Const>),
    MacroCall(FileItemTreeId<MacroCall>),
});

impl From<AssocItem> for ModItem {
    fn from(item: AssocItem) -> Self {
        match item {
            AssocItem::Function(it) => it.into(),
            AssocItem::TypeAlias(it) => it.into(),
            AssocItem::Const(it) => it.into(),
            AssocItem::MacroCall(it) => it.into(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Variant {
    pub name: Name,
    pub fields: Fields,
}

/// A range of densely allocated ItemTree IDs.
pub struct IdRange<T> {
    range: Range<u32>,
    _p: PhantomData<T>,
}

impl<T> IdRange<T> {
    fn new(range: Range<Idx<T>>) -> Self {
        Self { range: range.start.into_raw().into()..range.end.into_raw().into(), _p: PhantomData }
    }
}

impl<T> Iterator for IdRange<T> {
    type Item = Idx<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> DoubleEndedIterator for IdRange<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> fmt::Debug for IdRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("IdRange::<{}>", type_name::<T>())).field(&self.range).finish()
    }
}

impl<T> Clone for IdRange<T> {
    fn clone(&self) -> Self {
        Self { range: self.range.clone(), _p: PhantomData }
    }
}

impl<T> PartialEq for IdRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<T> Eq for IdRange<T> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fields {
    Record(IdRange<Field>),
    Tuple(IdRange<Field>),
    Unit,
}

/// A single field of an enum variant or struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Name,
    pub type_ref: Interned<TypeRef>,
    pub visibility: RawVisibilityId,
}
