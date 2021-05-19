module "region_1" {
    loadbalancer_peering_id = module.region_3_services.aws_loadbalancer_vpc_peer_id
    source = "./module_workloads"
    aws_region = "us-east-1"
    region_vpc_cidr = "10.10.0.0/16"
    region_subnet1_block = "10.10.1.0/24"
    region_subnet2_block = "10.10.2.0/24"
}

module "region_2" {
    loadbalancer_peering_id = module.region_3_services.aws_loadbalancer_vpc_peer_id
    source = "./module_workloads"
    aws_region = "us-west-1"
    region_vpc_cidr = "10.20.0.0/16"
    region_subnet1_block = "10.20.1.0/24"
    region_subnet2_block = "10.20.2.0/24"
}


module "region_3_services" {
    workloads_vpcs_ids = [module.region_1.workload_vpc_id , module.region_2.workload_vpc_id]
    source = "./module_services"
    aws_region = "sa-east-1"
    region_vpc_cidr = "10.30.0.0/16"
    region_subnet1_block = "10.30.1.0/24"
    region_subnet2_block = "10.30.2.0/24"
}