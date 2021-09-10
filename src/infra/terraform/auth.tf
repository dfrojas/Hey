variable "path_to_creds" {
  type = string
}


provider "aws" {
  region                  = "us-west-2"
  shared_credentials_file = var.path_to_creds
  profile                 = "hey"
}
