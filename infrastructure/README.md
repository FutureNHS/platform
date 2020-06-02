# Infrastructure

We use terraform to build our environments

## Installation Instructions

1. Install [Terraform](https://www.terraform.io/) 

    ```bash
    brew install terraform
    ```  
   
1. Install a Terraform version switcher 
    ([Terraform Switcher](https://github.com/warrensbox/terraform-switcher)
    or [CHTF](https://github.com/Yleisradio/homebrew-terraforms))

    ```bash
    brew install warrensbox/tap/tfswitch
    ```  
   
    or
    
    ```bash
    brew install chtf
    ```    
   
1. Select version 0.12.25 

    ```bash
    tfswitch 0.12.25
    ```  
   
    or
    
    ```bash
    chtf 0.12.25
    ```    
   
1. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest) 

    ```bash
    brew install azure-cli
    ```      
   
1. Login to Azure 

    ```bash
    az login
    ``` 
   
## Dev Environment Setup
 
Infrastructure is setup so every developer can create their own instance of the environment in Azure,
as opposed to sharing a staging environment.

1. Clone the FutureNHS Platform 

1. Create the new dev environment with __your name__ as the parameter

    If your name is __John__, your command is as follows:
    
    ```bash
    ./infrastructure/scripts/create-dev-environment.sh john
    ```

1. CD into your new dev folder

    ```bash
    cd infrastructure/environments/dev
    ``` 
  
1. Follow the onscreen instructions to create the terraform.tfvars file

    The contents of the file should still be printed on the screen

    Below is an example of what it would likely look like:
    
    ```hcl-terraform
    resource_group_name="tfstatejohn"
    storage_account_name="fnhstfstatedevjohn"
    USERNAME="john"
    ``` 

1. Run Terraform Init using the vars file you just created

    ```bash
    terraform init -backend-config=terraform.tfvars
    ``` 

1. Create an execution plan

    ```bash
    terraform plan
    ``` 

1. Apply changes and execute your plan on the server

    ```bash
    terraform apply
    ``` 

1. In order to use Kubernetes CLI (kubectl) commands, you need to pull the credentials from the server.
   
   As with step 2, if your name is __John__, your command will be: 

    ```bash
    az aks get-credentials --resource-group platform-dev-john --name dev-john
    ``` 

1. To reduce infrastructure costs for the NHS, 
please destroy your environment when you no longer need it.

    ```bash
    terraform destroy
    ``` 
