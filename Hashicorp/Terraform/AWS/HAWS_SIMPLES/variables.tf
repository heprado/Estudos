variable "tag_enviroment" {

}

variable "tag_project" {

}

variable "tag_team" {

}

variable webserver_vpc_cidr {
    description = "The CIDR that will be used for the Webservers private IPv4 address."
}

variable subnet1_cidr {
    description = "The CIDR that will be used for the webservers connected to this subnet, every subnet will be deployed in diffent AZs. "
}

variable subnet2_cidr {
    description = "The CIDR that will be used for the webservers connected to this subnet, every subnet will be deployed in diffent AZs. "
}

variable distro {
    description = "The distro that will be used for the Webservers, you can look it up in Amazon System Manager or aws-cli, by default this module uses the most recent."
}

variable instance_type {
    description = "The instance type of your EC2s"
}

variable volume_size {
    description = "The size of the root EBS Volume of your EC2s."
}

variable ssh_key_name {
    description = "Key name of an Key Pair created in your AWS account, if you don't have any just go to the EC2 Panel and create one and use the name here."
}

variable ec2_ami {
    description = "The AMI ID that will be used to deploy your EC2 Instances"
}