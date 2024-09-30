use axum::{extract::FromRequestParts, middleware::FromExtractorLayer, Router as GenericRouter};

pub type AxumRouterWithState<T> = GenericRouter<T>;
pub type StatefulRoutes<T> = Vec<StatefulRoute<T>>;
pub type StatefulRoute<T> = (&'static str, axum::Router<T>);

pub type StatefulMiddleware<E, S> = Option<Vec<FromExtractorLayer<E, S>>>;
pub trait StatefulNestedRouter<T> {
    fn get() -> StatefulRoute<T>;
}

pub struct Router<T> {
    app: GenericRouter,
    pub address: String,
    pub routes: StatefulRoutes<T>,
}

impl<T> Clone for Router<T> {
    fn clone(&self) -> Self {
        return Self {
            app: self.app.clone(),
            address: self.address.clone(),
            routes: self.routes.clone(),
        };
    }
}

impl<T: Clone + Send + Sync + 'static> Router<T> {
    pub fn new() -> Self {
        return Self {
            app: GenericRouter::new(),
            address: String::from("127.0.0.1:3000"),
            routes: vec![],
        };
    }

    pub async fn setup(
        &self,
        address: Option<String>,
        routes: Option<StatefulRoutes<T>>,
        state: T,
    ) -> Self {
        let mut self_clone = self.clone();

        if address.is_some() {
            self_clone.address = address.unwrap();
        }

        if routes.is_some() {
            self_clone.routes = routes.unwrap();
        }

        self_clone.app = AxumRouterWithState::new()
            .nest("/api", self_clone.get_routes())
            .with_state(state);

        return self_clone;
    }

    pub fn add_middleware<E: FromRequestParts<S> + 'static, S: Clone + Send + Sync + 'static>(
        &self,
        middleware: StatefulMiddleware<E, S>,
    ) -> Self {
        if middleware.is_some() {
            let mut self_clone = self.clone();
            let middleware_to_add = middleware.unwrap();

            for mw in middleware_to_add {
                self_clone.app = self_clone.app.route_layer(mw);
            }

            return self_clone;
        }
        return self.clone();
    }

    pub async fn serve(self) {
        //Create Listener to consume the API.
        let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();

        let routes = self.routes.clone();

        let route_names: Vec<String> = routes.iter().map(|route| format!("/{}", route.0)).collect();

        //Let the console know that we are now listening.
        tracing::info!("listening on {}", &self.address);
        tracing::debug!("API Routes: {:?}", route_names);

        //Serve the entire API.
        axum::serve(listener, self.app).await.unwrap();
    }

    fn get_routes(&self) -> AxumRouterWithState<T> {
        let mut api_routes = AxumRouterWithState::new();

        for router_with_endpoint in self.routes.clone() {
            let (endpoint, router) = router_with_endpoint;

            api_routes = api_routes.nest(format!("/{}", endpoint).as_str(), router);
        }

        return api_routes;
    }
}
