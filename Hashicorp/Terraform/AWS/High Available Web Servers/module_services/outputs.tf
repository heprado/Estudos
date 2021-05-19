output "aws_loadbalancer_vpc_id" {
    value = aws_vpc.region_vpc.id
}

output "aws_loadbalancer_vpc_peer_id" {
    value = toset([
        for peering in aws_vpc_peering_connection.vpc_peering_from_loadbalancer.id : peering.id
    ])
}