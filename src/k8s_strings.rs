pub(crate) fn getDeployment(name: String) -> String {
    let str: String = format!("{name}");

    let deployment: String = format!(
        r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {}
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.14.2
        ports:
        - containerPort: 80
"#,
        str
    );

    return deployment;
}
