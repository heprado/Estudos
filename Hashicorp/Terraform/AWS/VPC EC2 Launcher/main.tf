provider aws {
    region = var.aws_region
    shared_credentials_file = var.shared_credentials_path
    profile = "default"
}

/*Cria a chave publica e a privada no diretorio onde o Terraform está rodando.*/
resource "null_resource" "public_private_key" {
    provisioner "local-exec" {
        command = "ssh-keygen -f ${path.module}/${var.prefix_name}_key_pair_launcher -m pem -N ''"
        } 
    /*Remove a chave privada quando fizermos terraform destroy*/
    provisioner "local-exec" {
        when = destroy
        command = "rm ./*_key_pair_launcher"
    }
    /*Remove a chave publica quando fizermos terraform destroy*/
    provisioner "local-exec" {
        when = destroy
        command = "rm ./*key_pair_launcher.pub"
    }
    
}
/*Necessario pois não podemos usar a função file para ler a chave, funções não participam
da dependencia do Terraform.*/
data "local_file" "read_key" {
    depends_on = [null_resource.public_private_key]
    filename = "${path.module}/${var.prefix_name}_key_pair_launcher.pub"
}
/*Passando a chave publica que criamos para a AWS.*/
resource "aws_key_pair" "public_key" {
    key_name = "${var.prefix_name}_key_pair_launcher"
    public_key = data.local_file.read_key.content
}



