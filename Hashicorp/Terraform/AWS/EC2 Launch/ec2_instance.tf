provider aws {
    region = "sa-east-1"
    #Caminho do arquivo contendo a Access Key e Secret Access Key
    shared_credentials_file = "C:/Users/heprado/Documents/Github/Studies/Hashicorp/Terraform/AWS/EC2 Launch/credentials"
    profile="default"
}

resource "aws_instance" "aws_instance" {
    ami = "ami-0096398577720a4a3"
    key_name = "awskeypair_sp"
    subnet_id = "subnet-081c3786efc59c069"
    vpc_security_group_ids = [ "sg-07efeaf765e7534b3" ]
    associate_public_ip_address = true
    instance_type = "t3.micro"
    tags = {
        tier = "db"
        appname = "app2"
        Name = "APP_Terraform"
    }
}
