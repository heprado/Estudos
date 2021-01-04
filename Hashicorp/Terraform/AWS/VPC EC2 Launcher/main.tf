provider aws {
    region = var.aws_region
    shared_credentials = var.shared_credentials_path
    profile = "default"
}

resource "tls_private_key" ""


