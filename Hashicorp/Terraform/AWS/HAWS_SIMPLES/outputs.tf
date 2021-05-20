output "subnet1_az" {
    value = "The Subnet1 is deployed in the ${data.aws_availability_zones.available_zones.names[0]} Availability Zone."
}

output "subnet2_az" {
    value = "The Subnet2 is deployed in the ${data.aws_availability_zones.available_zones.names[1]} Availability Zone."
}

output "ami" {
    value = {
        AMI_ID = data.aws_ami.ec2_ami.image_id
        Name = data.aws_ami.ec2_ami.name
    }
}

output "ec2_1_config" {
    value = {
        Availability_Zone = data.aws_availability_zones.available_zones.names[0]
        Public_IPv4 = aws_instance.webserver_1.public_ip
    }
}

output "ec2_2_config" {
    value = {
        Availability_Zone = data.aws_availability_zones.available_zones.names[1]
        Public_IPv4 = aws_instance.webserver_2.public_ip
    }
}