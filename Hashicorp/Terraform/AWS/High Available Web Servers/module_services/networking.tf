locals {
    commom_tags = {
        Project = var.project
        Enviroment = var.enviroment
    }
}


data "aws_availability_zones" "available" {
  state = "available"
}

resource "aws_vpc" "region_vpc" {
    cidr_block = var.region_vpc_cidr
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${var.region_vpc_cidr}-vpc"})
}


resource "aws_subnet" "region_subnet1" {
    cidr_block = var.region_subnet1_block
    map_public_ip_on_launch = "false"
    vpc_id = aws_vpc.region_vpc.id
    availability_zone = data.aws_availability_zones.available.names[0]
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${data.aws_availability_zones.available.names[0]}-subnet"})
}

resource "aws_subnet" "region_subnet2" {
    cidr_block = var.region_subnet2_block
    map_public_ip_on_launch = "false"
    vpc_id = aws_vpc.region_vpc.id
    availability_zone = data.aws_availability_zones.available.names[1]
    tags = merge(local.commom_tags, {Name = "${var.prefix}-${data.aws_availability_zones.available.names[1]}-subnet"})
}

resource "aws_vpc_peering_connection" "vpc_peering_from_loadbalancer" {
  peer_vpc_id   = var.workloads_vpcs_ids
  vpc_id        = aws_vpc.region_vpc.id 
  peer_region   = "us-east-1"
}