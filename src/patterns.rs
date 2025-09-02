// Default gitignore patterns applied in addition to the project's .gitignore files
pub const DEFAULT_IGNORES: &[&str] = &[
    // 🌐 General
    ".env",
    ".env.*",
    "*.secret",
    "*.secrets.*",
    "*.keys",
    "*.key",
    "*.pem",
    "*.crt",
    "secrets.*",
    "credentials.*",
    "auth.*",
    "config.local.*",
    "settings.local.*",
    // 🟦 Node.js / JS / TS
    ".npmrc",
    ".npmrc.*",
    "config/local.*",
    "firebase-service-account.json",
    // 🟩 Python
    "local_settings.py",
    "settings_local.py",
    "secrets.json",
    // 🔷 Ruby / Rails
    "config/master.key",
    "config/credentials.yml.enc",
    "config/secrets.yml",
    // ☕ Java / Kotlin / Spring Boot
    "application-local.*",
    "application-dev.*",
    "secrets.properties",
    // 🐘 PHP / Laravel / Symfony
    ".env.*.local",
    "config/secrets/*/*/*.php",
    // 🟦 .NET / C#
    "appsettings.Development.json",
    "appsettings.*.json",
    "secrets.json",
    // 🐳 Docker / Kubernetes / Infrastructure
    "docker-compose.override.yml",
    "*.override.yml",
    "kubeconfig",
    "secrets.yaml",
    "terraform.tfvars",
    "terraform.tfvars.json",
    ".terraform/*",
    // 📱 Mobile
    "google-services.json",
    "GoogleService-Info.plist",
    "*.jks",
    "*.p12",
];
