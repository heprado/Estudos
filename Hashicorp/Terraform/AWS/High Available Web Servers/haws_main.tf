provider aws{
    region = var.provider_config["region"]
    shared_credentials_file = var.provider_config["credentials_path"]
    profile = var.provider_config["profile"]
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
da dependencia do Terraform. Só serve para termos um objeto no terraform para podermos controlar a ordem das coisas*/
data "local_file" "read_key" {
    /*Depends_on para primeiro a chave ser criada.*/
    depends_on = [null_resource.public_private_key]
    filename = "${path.module}/${var.prefix_name}_key_pair_launcher.pub"
}

/*Passando a chave publica que criamos para a AWS.*/
resource "aws_key_pair" "public_key" {
    key_name = "${var.prefix_name}_key_pair_launcher"
    /*Se usarmos a função file para ler o arquivo o terraform le antes de criarmos ela, pois funções não participam das 
    dependencias do terraform*/
    public_key = data.local_file.read_key.content
}

/*Criando o VPC*/
resource "aws_vpc" "vpc" {
    cidr_block = var.vpc_cidr_block
    tags = {
        "Name" = "${var.prefix_name}_vpc"
    }
}

/*Criando subnets*/

resource "aws_subnet" "subnets" {
    /*Sempre precisamos passar um set de strings, nesse caso a variavel no arquivo de variaveis já está como type=set(string)*/
    for_each = toset(var.subnets)
    vpc_id = aws_vpc.vpc.id
    cidr_block = each.value
    map_public_ip_on_launch = true
    tags = {
        "Name" = "${var.prefix_name}_${each.value}_vpc"
    }
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.vpc.id

  tags = {
    "Name" = "${var.prefix_name}_igw"
  }
}

/*Criamos uma net_acl liberando tudo de fora pra dentro, pois a melhor pratica é fazer a segurança nos security groups*/
resource "aws_network_acl" "net_acl" {
  vpc_id = aws_vpc.vpc.id
  /*Para cada subnet na resource aws_subnet.subnets o id equivale aos ids das subnets*/
  subnet_ids = [for subnet in aws_subnet.subnets : subnet.id]
  egress {
    protocol   = "-1"
    rule_no    = 200
    action     = "allow"
    cidr_block = var.vpc_cidr_block
    from_port  = 0
    to_port    = 0
  }

  ingress {
    protocol   = "-1"
    rule_no    = 200
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }
  tags = {
      "Name" = "${var.prefix_name}_net-acl"
  }
}

