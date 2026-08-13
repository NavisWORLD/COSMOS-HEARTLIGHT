# Security Policy

HEARTLIGHT is local-first reference software. Do not deploy it with identifiable student records on an unmanaged public host.

For production school deployments:

- use HTTPS;
- perform district security/privacy review;
- define access control and retention;
- encrypt server-side records if a server is added;
- never commit student records, API keys, or school credentials to source control;
- disable debug logging of student content;
- verify camera permissions and data flows on every supported platform.
