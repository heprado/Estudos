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

/*Creating the first Webserver*/

resource "aws_instance" "webserver_1" {
    key_name = var.ssh_key_name
    ami = var.ec2_ami
    iam_instance_profile = aws_iam_instance_profile.webservers_profile.name
    instance_type = var.instance_type
    vpc_security_group_ids = [aws_security_group.ec2_sg.id]
    subnet_id = aws_subnet.subnet1.id
    
    root_block_device {
        volume_size = var.volume_size
    }

    ##Using the connection resource to connect to the EC2
    connection {
    type        = "ssh"
    host        = self.public_ip
    user        = "ubuntu"
    private_key = file("${path.root}/ssh_keys/${var.ssh_key_name}.pem")

  }

  ##This will create a file inside the EC2, this file will be used by s3cmd to write the nginx logs into the S3 Bucket, and to download the HTML files
  ##s3cmd get the access key and secret key magically with the metadata of the EC2 Instance Profile.
    provisioner "file" {
    content = <<EOF
  access_key =
  secret_key =
  security_token =

  EOF

    destination = "~/.s3cfg"
  }

  provisioner "file" {
    content = <<EOF
  /var/log/nginx/*log {
    daily
    rotate 10
    missingok
    dateext
    compress
    sharedscripts
    postrotate
    endscript
    lastaction
        INSTANCE_ID=`curl --silent http://169.254.169.254/latest/meta-data/instance-id`
        sudo s3cmd sync --config=~/.s3cfg /var/log/nginx/ s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/nginx/$INSTANCE_ID/
    endscript
}
EOF


    destination = "~/nginx"
  }


  provisioner "remote-exec" {
    inline = [
      "sudo apt update",
      "sudo apt-get update",
      "sudo apt-get install nginx pip -y",
      "sudo apt upgrade -y ",
      "sudo pip install s3cmd",
      "sudo cp ~/nginx /etc/logrotate.d/nginx",
      "s3cmd get s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/website/index.html .",
      "s3cmd get s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/website/chuu.jfif .",
      "sudo rm /var/www/html/* ",
      "sudo cp ~/index.html /var/www/html/index.html",
      "sudo cp ~/chuu.jfif /var/www/html/chuu.jfif",
      "sudo service nginx start",
      "sudo logrotate -f /etc/logrotate.conf"
    ]
  }
}
/*Creating the first Webserver*/

resource "aws_instance" "webserver_2" {

    key_name = var.ssh_key_name
    ami = var.ec2_ami
    iam_instance_profile = aws_iam_instance_profile.webservers_profile.name
    instance_type = var.instance_type
    vpc_security_group_ids = [aws_security_group.ec2_sg.id]
    subnet_id = aws_subnet.subnet2.id


    root_block_device {
        volume_size = var.volume_size
    }
  ##Using the connection resource to connect to the EC2
  connection {
    type        = "ssh"
    host        = self.public_ip
    user        = "ubuntu"
    private_key = file("${path.root}/ssh_keys/${var.ssh_key_name}.pem")

  }

  provisioner "file" {
    content = <<EOF
  access_key =
  secret_key =
  security_token =
  use_https = True

  EOF

    destination = "~/.s3cfg"
  }

  provisioner "file" {
    content = <<EOF
  /var/log/nginx/*log {
    daily
    rotate 10
    missingok
    dateext
    compress
    sharedscripts
    postrotate
    endscript
    lastaction
        INSTANCE_ID=`curl --silent http://169.254.169.254/latest/meta-data/instance-id`
        sudo s3cmd sync --config=~/.s3cfg /var/log/nginx/ s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/nginx/$INSTANCE_ID/
    endscript
}
EOF


    destination = "~/nginx"
  }


  provisioner "remote-exec" {
    inline = [
      "sudo apt update",
      "sudo apt-get update",
      "sudo apt-get install nginx pip -y",
      "sudo apt upgrade -y ",
      "sudo pip install s3cmd",
      "sudo cp ~/nginx /etc/logrotate.d/nginx",
      "s3cmd get s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/website/index.html .",
      "s3cmd get s3://${aws_s3_bucket.logs_htmlstatic_s3.id}/website/chuu.jfif .",
      "sudo rm /var/www/html/* ",
      "sudo cp ~/index.html /var/www/html/index.html",
      "sudo cp ~/chuu.jfif /var/www/html/chuu.jfif",
      "sudo service nginx start",
      "sudo logrotate -f /etc/logrotate.conf"
    ]
  }
}

/* ############################ S3 Bucket ############################ */

/*Creating the S3 bucket for the logs and static html files, we are creating it as private but we will give access to them with IAM Policies*/
resource "aws_s3_bucket" "logs_htmlstatic_s3" {

    bucket = local.bucket_random_name
    acl    = "private"
    force_destroy = true
    tags = merge(local.tags, {Name : "${var.tag_project}-s3_bucket"})

}

resource "aws_s3_bucket_object" "index" {
    bucket = aws_s3_bucket.logs_htmlstatic_s3.bucket
    key = "/website/index.html"
    source = "${path.root}/HTML/index.html"

  }

  resource "aws_s3_bucket_object" "image" {
    bucket = aws_s3_bucket.logs_htmlstatic_s3.bucket
    key = "/website/chuu.jfif"
    source = "${path.root}/HTML/chuu.jfif"

  }


/* ############################ IAM Configuration ############################ */

/*This IAM Role let the EC2 service to assume the role.*/
resource "aws_iam_role" "ec2_role" {
  name        = "${var.tag_project}-ec2_role"
  path        = "/"
  description = "This let the EC2s instances to assume this role"

  /*This is the assume role, this let the EC2 service to assume this role.*/
  assume_role_policy = jsonencode({
  Version =  "2012-10-17",
  Statement = [
      {   Sid = "",
          Effect =  "Allow",
          Action =  "sts:AssumeRole",
          Principal = {
              Service = "ec2.amazonaws.com"
          }
           
      }
  ]
})  

tags = merge(local.tags, {Name : "${var.tag_project}-"})
}

/*This create a policy inside the role above, this policy let our EC2s do everything it needs on the S3 Bucket*/
resource "aws_iam_role_policy" "allow_s3_all" {
  name = "${var.tag_project}-allow_s3_all_policy"
  /*Passing the role that this policy will be created*/
  role = aws_iam_role.ec2_role.name
  /*The policy itself.*/
  policy = jsonencode(
    {
  Version =  "2012-10-17",
  Statement = [
    {
      Action =  [
        "s3:*"
      ],
      Effect = "Allow",
      Resource = [
                "arn:aws:s3:::${aws_s3_bucket.logs_htmlstatic_s3.bucket}",
                "arn:aws:s3:::${aws_s3_bucket.logs_htmlstatic_s3.bucket}/*"
            ]
    }
  ]
}
)
}

/*This create a IAM Instance Profile, so the EC2 can access the Role that will give access to the S3 Bucket*/
resource "aws_iam_instance_profile" "webservers_profile" {
  name = "${var.tag_project}-webservers_profile"
  role = aws_iam_role.ec2_role.name

  tags = merge(local.tags, {Name : "${var.tag_project}-webserver_profile"})
}

