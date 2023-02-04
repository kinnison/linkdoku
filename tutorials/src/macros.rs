//! Macros for tutorials
//!

#[macro_export]
macro_rules! tutorial {
    ($name:ident, $( $id:ident: $text:expr ),*) => {
        $crate::tutorial!(@build_struct ($($id),*) -> {struct $name });

        impl std::default::Default for $name {
            fn default() -> Self {
                $crate::tutorial!(@build_init ($($id),*) -> {Self})
            }
        }

        impl $name {
            $(
                $crate::tutorial!(@setter $id);
            )*
        }
        $crate::tutorial!(@builder $name -> $(($id $text))*);
    };

    (@builder $name:ident -> $(($id:ident $text:expr))*) => {
        impl From<$name> for $crate::TutorialData {
            fn from(mut value: $name) -> $crate::TutorialData {
                let mut ret = $crate::TutorialData::new(stringify!($name));
                $(
                    if let Some(node) = value.$id.take() {
                        ret.add_node(node, stringify!($id), $text);
                    }
                )*
                ret
            }
        }
    };

    (@build_struct ( $name:ident, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::tutorial!(@build_struct ($($next)*) -> {$($output)* ($name: Option<yew::NodeRef>)});
    };

    (@build_struct ( $name:ident ) -> {$($output:tt)*}) => {
        $crate::tutorial!(@build_struct () -> {$($output)* ($name: Option<yew::NodeRef>)});
    };

    (@build_struct () -> {struct $name:ident $(($id:ident: $ty:ty))*}) => {
        struct $name {
            $($id: $ty),*
        }
    };

    (@build_init ($name:ident, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::tutorial!(@build_init ($($next)*) -> {$($output)* ($name)});
    };

    (@build_init ($name:ident) -> {$($output:tt)*}) => {
        $crate::tutorial!(@build_init () -> {$($output)* ($name)});
    };

    (@build_init () -> {Self $(($id:ident))*}) => {
        Self {
            $(
                $id: None,
            )*
        }
    };

    (@setter $id:ident) => {
        fn $id(&mut self, node: yew::NodeRef) -> &mut Self {
            self.$id = Some(node);
            self
        }
    }

}

#[macro_export]
macro_rules! use_tutorial_node {
    ($tutorial:ident . $part:ident) => {{
        struct TutorialNodeSetter<T> {
            tutorial: T,
        };
        impl<T> ::yew::functional::Hook for TutorialNodeSetter<T>
        where
            T: FnOnce(NodeRef),
        {
            type Output = ::yew::NodeRef;
            fn run(self, ctx: &mut ::yew::functional::HookContext) -> Self::Output {
                let node = ::yew::functional::Hook::run(use_node_ref(), ctx);
                (self.tutorial)(node.clone());
                node
            }
        }
        TutorialNodeSetter {
            tutorial: |node| {
                $tutorial.$part(node);
            },
        }
    }};
}
