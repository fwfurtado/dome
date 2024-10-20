#[macro_export]
macro_rules! routes {
    ($($prefix:expr => $routes:expr),*) => {
        {
            use axum::Router;
            let mut router = Router::new();
            $(
                let router = router.nest($prefix, $routes);
            )*

            router
        }
    };
}
