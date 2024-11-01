pub(crate) struct cParams {
    apiVersion: String,
    metadata_name: String,
    metadata_labels_app: String,
    spec_replicas: u32,
    spec_selector_matchLabels_app: String,
    spec_template_metadata_labels_app: String,
    spec_template_spec_containers_name: String,
    spec_template_spec_containers_image: String,
    spec_template_spec_containers_ports_containerPort: u32,
}

impl cParams {
    pub(crate) fn new() -> cParams {
        cParams {
            apiVersion: String::from("apps/v1"),
            metadata_name: String::new(),
            metadata_labels_app: String::new(),
            spec_replicas: 1,
            spec_selector_matchLabels_app: String::new(),
            spec_template_metadata_labels_app: String::new(),
            spec_template_spec_containers_name: String::new(),
            spec_template_spec_containers_image: String::new(),
            spec_template_spec_containers_ports_containerPort: 8080,
        }
    }

    pub(crate) fn get_deployment(&mut self) -> String {
        let deployment: String = format!(
            r#"
apiVersion: {0}
kind: Deployment
metadata:
  name: {1}
  labels:
    app: {2}
spec: 
  replicas: {3}
  selector:
    matchLabels:
      app: {4}
  template:
    metadata:
      labels:
        app: {5}
    spec:
      containers:
      - name: {6}
        image: {7}
        ports:
        - containerPort: {8}
    "#,
            self.apiVersion,
            self.metadata_name,
            self.metadata_labels_app,
            self.spec_replicas,
            self.spec_selector_matchLabels_app,
            self.spec_template_metadata_labels_app,
            self.spec_template_spec_containers_name,
            self.spec_template_spec_containers_image,
            self.spec_template_spec_containers_ports_containerPort,
        );

        return deployment;
    }
}
