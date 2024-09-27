use axum::Router as GenericRouter;

pub type AxumRouter = GenericRouter;
pub type Routes = Vec<Route>;
pub type Route = (&'static str, GenericRouter);

pub trait NestedRouter {
    fn get() -> Route;
}
pub struct Router {
    app: GenericRouter,
    pub address: String,
    pub routes: Routes,
}

impl Clone for Router {
    fn clone(&self) -> Self {
        return Self {
            app: self.app.clone(),
            address: self.address.clone(),
            routes: self.routes.clone(),
        };
    }
}

impl Router {
    pub fn new() -> Self {
        return Self {
            app: GenericRouter::new(),
            address: String::from("127.0.0.1:3000"),
            routes: vec![],
        };
    }

    pub async fn setup(&mut self, routes: Option<Routes>) -> Self {
        let mut self_clone = self.clone();

        if routes.is_some() {
            self_clone.routes = routes.unwrap();
        }

        self_clone.app = AxumRouter::new().nest("/api", self_clone.get_routes());

        return self_clone;
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

    fn get_routes(&self) -> AxumRouter {
        let mut api_routes = AxumRouter::new();

        for router_with_endpoint in self.routes.clone() {
            let (endpoint, router) = router_with_endpoint;

            api_routes = api_routes.nest(format!("/{}", endpoint).as_str(), router);
        }

        return api_routes;
    }
}
