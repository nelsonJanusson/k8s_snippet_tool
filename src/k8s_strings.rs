pub(crate) fn get_deployment(name: String) -> String {
    let str: String = format!("{name}");

    let deployment: String = format!(
        r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {0}
  labels:
    app: {0}
spec:
  replicas: 3
  selector:
    matchLabels:
      app: {0}
  template:
    metadata:
      labels:
        app: {0}
    spec:
      containers:
      - name: {0}
        image: {0}
        ports:
        - containerPort: 80
"#,
        str
    );

    return deployment;
}
