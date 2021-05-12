locals {
    commom_tags = {
        Project = var.project
        Enviroment = var.enviroment
    }
}

resource "aws_vpc" "region1_vpc" {
    cidr_block = var.region1_vpc_cidr
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${var.region1_vpc_cidr}-vpc"})
}


resource "aws_subnet" "region1_subnet1" {
    cidr_block = var.region1_subnet1_block
    map_public_ip_on_launch = "false"
    vpc_id = aws_vpc.region1_vpc.id
    availability_zone = "${var.aws_region}a"
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${var.aws_region}a-subnet"})
}

resource "aws_subnet" "region1_subnet2" {
    cidr_block = var.region1_subnet2_block
    map_public_ip_on_launch = "false"
    vpc_id = aws_vpc.region1_vpc.id
    availability_zone = "${var.aws_region}b"
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${var.aws_region}b-subnet"})
}

