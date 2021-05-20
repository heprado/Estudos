

locals {
    ##Normalizing the variables inside a map
    tags = {
        Enviroment = var.tag_enviroment
        Team = var.tag_team
    }

    ##The bucket name
    bucket_random_name = lower("${var.tag_project}-${random_integer.bucket_randomizer.id}")
}

/*This resource generates an random integer that will be concatenated into the S3 Bucket name, because every S3 Bucket needs a unique name*/
resource "random_integer" "bucket_randomizer" {
    min = 1111
    max = 9999
}

/* ############################ NETWORKING ############################ */


/*Creating the VPC for the Webservers*/

resource "aws_vpc" "vpc" {
    cidr_block = var.webserver_vpc_cidr

    tags = merge(local.tags, {Name : "${var.tag_project}-vpc"})
}

/*Getting the available AZs on AWS, so that we can create the subnets in different AZs*/

data "aws_availability_zones" "available_zones" {
    state = "available"
}

/*Creating one subnet in an Availability Zone diffent from the other, so the EC2s that we will deploy will be highly available*/
resource "aws_subnet" "subnet1" {
    availability_zone = data.aws_availability_zones.available_zones.names[0]
    vpc_id = aws_vpc.vpc.id
    map_public_ip_on_launch = true
    cidr_block = var.subnet1_cidr
    tags = merge(local.tags, {Name : "${var.tag_project}-subnet1"})
}

/*Creating one subnet in an Availability Zone diffent from the other, so the EC2s that we will deploy will be highly available*/
resource "aws_subnet" "subnet2" {
    availability_zone = data.aws_availability_zones.available_zones.names[1]
    vpc_id = aws_vpc.vpc.id
    map_public_ip_on_launch = true
    cidr_block = var.subnet2_cidr
    tags = merge(local.tags, {Name : "${var.tag_project}-subnet2"})
}

/*Creating the Internet Gateway so that our EC2s can access the internet*/
resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.vpc.id

  tags = merge(local.tags, {Name : "${var.tag_project}-igw"})
}

/*Creating the Routing Table for our VPC*/
resource "aws_route_table" "route_table" {
  vpc_id = aws_vpc.vpc.id
  tags = merge(local.tags, {Name : "${var.tag_project}-route_table"})
}

/*Creating the default route for our Routing Table, so the VPC knows how to reach the internet.*/
resource "aws_route" "default_route" {
  route_table_id            = aws_route_table.route_table.id
  destination_cidr_block    = "0.0.0.0/0"
  gateway_id = aws_internet_gateway.igw.id
}

/*Making the routing table that we created the main route table for the VPC, this will make the subnets associated automatically to the routing table that 
we created*/
resource "aws_main_route_table_association" "main_route_table" {
  vpc_id         = aws_vpc.vpc.id
  route_table_id = aws_route_table.route_table.id
}

/*Security Group that the EC2s will use*/
resource "aws_security_group" "ec2_sg" {
  vpc_id = aws_vpc.vpc.id
  name = "${var.tag_project}-ec2_sg"

  ##Permiting HTTPS inbound from all the internet on our EC2s
  ingress {
    description = "Permit HTTPS from the Internet to the Security Group"
    from_port = 443
    to_port = 443
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ##Permiting HTTP inbound from all the internet on our EC2s
  ingress {
    description = "Permit HTTP from the Internet to the Security Group"
    from_port = 80
    to_port = 80
    protocol = "tcp"
    cidr_blocks      = ["0.0.0.0/0"]
  }

  ##Permiting SSH inbound from all the internet on our EC2s
  ingress {
    description = "Permit SSH from the Internet to the Security Group"
    from_port = 22
    to_port = 22
    protocol = "tcp"
    cidr_blocks  = ["0.0.0.0/0"]
  }

  ##Permiting everything outbound from our EC2s.
  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(local.tags, {Name : "${var.tag_project}-ec2_sg"})
}

/* ############################ COMPUTING ############################ */


/*Getting the AMI that we will use to deploy the EC2s*/
data "aws_ami" "ec2_ami" {
  most_recent      = true
  owners           = ["self","amazon","aws-marketplace"]

  filter {
    name   = "name"
    values = ["${var.distro}*"]
  }

  filter {
    name   = "root-device-type"
    values = ["ebs"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }
}


/*Creating the first Webserver*/

resource "aws_instance" "webserver_1" {
    key_name = var.ssh_key_name
    ami = data.aws_ami.ec2_ami.id
    instance_type = var.instance_type
    vpc_security_group_ids = [aws_security_group.ec2_sg.id]
    subnet_id = aws_subnet.subnet1.id
    
    root_block_device {
        volume_size = var.volume_size
    }
}

/*Creating the first Webserver*/

resource "aws_instance" "webserver_2" {

    key_name = var.ssh_key_name
    ami = data.aws_ami.ec2_ami.id
    instance_type = var.instance_type
    vpc_security_group_ids = [aws_security_group.ec2_sg.id]
    subnet_id = aws_subnet.subnet2.id


    root_block_device {
        volume_size = var.volume_size
    }
}

/* ############################ S3 Bucket ############################ */

resource "aws_s3_bucket" "logs_htmlstatic_s3" {

    bucket = local.bucket_random_name
    acl    = "private"

    tags = merge(local.tags, {Name : "${var.tag_project}-s3_bucket"})

}


/* ############################ IAM Configuration ############################ */

resource "aws_iam_policy" "ec2_policy" {
  name        = "ec2_policy"
  path        = "/"
  description = "This policy let the EC2s instances to assume this role"

  
  policy = jsonencode(
        {
          "Version": "2012-10-17",
          "Statement": [
            {
              "Action": "sts:AssumeRole",
              "Principal": {
                "Service": "ec2.amazonaws.com"
              },
              "Effect": "Allow",
              "Sid": ""
            }
          ]
        }
    )
}
/*Arrumar a bagaça abaixo*/
/*error creating IAM policy ec2_policy: MalformedPolicyDocument: Policy document should not specify a principal*/