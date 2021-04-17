resource "aws_dynamodb_table" "hey-animals" {
  name           = "Animals"
  billing_mode   = "PAY_PER_REQUEST"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "Name"
  range_key      = "Country"

  attribute {
    name = "Name"
    type = "S"
  }

  // TF only allow us B, N and S data types
  // How to set and M, SS or any other data type
  // supported by the Dynamo's API?
  attribute {
    name = "Country"
    type = "S"
  }

  attribute {
    name = "Population"
    type = "N"
  }

  // We need to change one hour if we want to change this value
  // https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateTimeToLive.html
  ttl {
    attribute_name = "TimeToExist"
    enabled        = false
  }

  global_secondary_index {
    name               = "AnimalNameIndex"
    hash_key           = "Country"
    range_key          = "Population"
    write_capacity     = 1
    read_capacity      = 1
    projection_type    = "KEYS_ONLY"
    # non_key_attributes = []
  }

}
