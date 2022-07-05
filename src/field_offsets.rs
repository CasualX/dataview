
#[doc(hidden)]
#[macro_export]
macro_rules! __field_offsets {
	(
		$(#$meta:tt)*
		$vis:vis struct $name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_name:ident: $field_ty:ty
			),*
			$(,)?
		}
	) => {
		const _: () = {
			#[derive(Copy, Clone, Debug)]
			$vis struct FieldOffsets {
				$($field_vis $field_name: usize,)*
			}
			impl $name where Self: $crate::Pod {
				const FIELD_OFFSETS: FieldOffsets = $crate::__field_offsets_impl!(0usize; {} $($field_name: $field_ty,)*);
			}
		};
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __field_offsets_impl {
	(
		$offset:expr;
		{$($init_name:ident: $init_expr:expr,)*}
		$field_name:ident: $field_ty:ty,
		$($tail_name:ident: $tail_ty:ty,)*
	) => {
		$crate::__field_offsets_impl!(
			$offset + ::core::mem::size_of::<$field_ty>();
			{ $($init_name: $init_expr,)* $field_name: $offset, }
			$($tail_name: $tail_ty,)*
		)
	};
	(
		$offset:expr;
		{$($init_name:ident: $init_expr:expr,)*}
	) => {
		FieldOffsets {
			$($init_name: $init_expr,)*
		}
	};
}
