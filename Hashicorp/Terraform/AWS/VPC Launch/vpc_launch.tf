
provider aws {
    region = var.region
    #Caminho do arquivo contendo a Access Key e Secret Access Key
    shared_credentials_file = "C:/Users/heprado/Documents/Github/Studies/Hashicorp/Terraform/AWS/VPC Launch/credentials"
    profile="default"
}

##Criando o VPC com o CIDR 
resource "aws_vpc" "heprado_vpc" {

    cidr_block = var.vpc_cidr

    tags = {
        Name = "${var.prefix_name}_vpc"
    }

}

##Criando a subnet e atrelando ela com o VPC, também coloquei para todas as EC2s nesse VPC pegarem IP publico automaticamente
resource "aws_subnet" "heprado_subnet"{
    vpc_id = aws_vpc.heprado_vpc.id
    cidr_block = var.subnet
    map_public_ip_on_launch = "true"
}

##Criando o Internet Gateway
resource "aws_internet_gateway" "heprado_igw" {
  vpc_id = aws_vpc.heprado_vpc.id

  tags = {
    Name = "${var.prefix_name}_igw"
  }
}
##Criando o security group e associando ao VPC
##Permitindo SSH e TCP para dentro do Security Group
##Permitindo tudo para fora do Security Group
##Tomar cuidado com o ICMP, quando o tipo do protocolo for ICMP as portas querem dizer o ICMP Type.
resource "aws_security_group" "allow_all_sg" {
  name        = "${var.prefix_name}_sg"
  description = "Allow all traffic ingress/egress"
  vpc_id      = aws_vpc.heprado_vpc.id

   ingress {
    description = "all_to_vpc"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    description = "all_from_vpc"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "${var.prefix_name}_sg"
  }
}

##Criando uma ACL que permite tudo, associando ela ao VPC que criamos e a subnet que criamos dentro dele.
##Essa ACL só funciona para essa Subnet especifica.
resource "aws_network_acl" "heprado_acl" {
  vpc_id = aws_vpc.heprado_vpc.id
  subnet_ids = [aws_subnet.heprado_subnet.id]

  egress {
    protocol   = "-1"
    rule_no    = 100
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }

  ingress {
    protocol   = "-1"
    rule_no    = 100
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }

  tags = {
    Name = "${var.prefix_name}_acl"
  }
}

##Criando uma route table para o VPC com uma rota default pro Internet Gateway.
resource "aws_route_table" "heprado_route" {
  vpc_id = aws_vpc.heprado_vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.heprado_igw.id
  }
}

##Associando a rota criada acima a subnet.
resource "aws_route_table_association" "heprado_route_association" {
  subnet_id      = aws_subnet.heprado_subnet.id
  route_table_id = aws_route_table.heprado_route.id
}

resource "aws_key_pair" "deployer" {
  key_name   = "heprado-sp_key"
  public_key = file("C:/Users/heprado/Documents/Github/Studies/Hashicorp/Terraform/AWS/VPC Launch/terraform_ec2_key.pub")
}

resource "aws_instance" "myfirst_instance" {
    ami = "ami-0c3c87b7d583d618f"
    vpc_security_group_ids = [aws_security_group.allow_all_sg.id]
    key_name = aws_key_pair.deployer.key_name
    subnet_id = aws_subnet.heprado_subnet.id
    associate_public_ip_address = true
    instance_type = "t3.micro"
    tags = {
        Name = "Santander-HOST1"
    }
}







