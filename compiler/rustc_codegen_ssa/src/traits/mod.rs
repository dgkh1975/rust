//! Interface of a Rust codegen backend
//!
//! This crate defines all the traits that have to be implemented by a codegen backend in order to
//! use the backend-agnostic codegen code in `rustc_codegen_ssa`.
//!
//! The interface is designed around two backend-specific data structures, the codegen context and
//! the builder. The codegen context is supposed to be read-only after its creation and during the
//! actual codegen, while the builder stores the information about the function during codegen and
//! is used to produce the instructions of the backend IR.
//!
//! The traits contain associated types that are backend-specific, such as the backend's value or
//! basic blocks.

mod abi;
mod asm;
mod backend;
mod builder;
mod consts;
mod coverageinfo;
mod debuginfo;
mod declare;
mod intrinsic;
mod misc;
mod statics;
mod type_;
mod write;

use std::fmt;

use rustc_middle::ty::layout::{HasParamEnv, HasTyCtxt};
use rustc_target::spec::HasTargetSpec;

pub use self::abi::AbiBuilderMethods;
pub use self::asm::{AsmBuilderMethods, AsmMethods, GlobalAsmOperandRef, InlineAsmOperandRef};
pub use self::backend::{BackendTypes, CodegenBackend, ExtraBackendMethods};
pub use self::builder::{BuilderMethods, OverflowOp};
pub use self::consts::ConstMethods;
pub use self::coverageinfo::CoverageInfoBuilderMethods;
pub use self::debuginfo::{DebugInfoBuilderMethods, DebugInfoMethods};
pub use self::declare::PreDefineMethods;
pub use self::intrinsic::IntrinsicCallMethods;
pub use self::misc::MiscMethods;
pub use self::statics::{StaticBuilderMethods, StaticMethods};
pub use self::type_::{
    ArgAbiMethods, BaseTypeMethods, DerivedTypeMethods, LayoutTypeMethods, TypeMembershipMethods,
    TypeMethods,
};
pub use self::write::{ModuleBufferMethods, ThinBufferMethods, WriteBackendMethods};

pub trait CodegenObject = Copy + PartialEq + fmt::Debug;

pub trait CodegenMethods<'tcx> = TypeMethods<'tcx>
    + MiscMethods<'tcx>
    + ConstMethods<'tcx>
    + StaticMethods
    + DebugInfoMethods<'tcx>
    + AsmMethods<'tcx>
    + PreDefineMethods<'tcx>
    + HasParamEnv<'tcx>
    + HasTyCtxt<'tcx>
    + HasTargetSpec;
