## Working with Sealed Secrets

The `kubeseal` utility uses [asymmetric crypto](https://www.futurelearn.com/courses/encryption-and-cryptography/0/steps/64720) to encrypt secrets that only the `sealed secret controller` can decrypt. In development, we encypt those secrets with the certificate that we retrieve from the [shared Azure Key Vault](https://portal.azure.com/#@red-badger.com/resource/subscriptions/4a4be66c-9000-4906-8253-6a73f09f418d/resourceGroups/vault/providers/Microsoft.KeyVault/vaults/fnhs-shared-dev/overview).

### Fetch the Sealed Secret Signing Certificate
You will need the Signing Certificate for reading existing sealed secrets, as well as creating new ones. Fetch the Sealed Secret secret from the shared dev key vault and apply to your dev cluster:
```
az keyvault secret show --vault-name "fnhs-shared-dev" --name "sealed-secret-yaml" | jq -r '.value' | kubectl apply -f -
```

### Creating new Sealed Secrets

Create a yaml-encoded Kubernetes Secret: (note use of `--dry-run` - this is just a local file and should be deleted afterwards)
```
$ kubectl create secret generic favouritecuisine \
   --dry-run \
   --namespace=hello-world \
   --from-literal=favouritecuisine=spanish \
   -o yaml \
   | kubeseal --format yaml > favouritecuisine-sealed.yaml
```
`favourite-cuisine-sealed.yaml` should now be added to `<app>manifests/dev-template` directory, and the file referenced in dev-template/kustomisation.yaml

Turn off argocd syncing to stop argocd syncing with deployments / master branch whilst you work on your app:
```
argocd app set hello-world --sync-policy none
```

Now regenerate the dev-overlays and push to argo-cd
```
./infrastructure/scripts/create-dev-overlays.py && argocd app sync hello-world --local ./hello-world/manifests/dev-matt --prune
```

You can verify the secret is in the namespace with:
```
kubectl get secrets -n hello-world
```

#### Use Secret to generate an environment variable
1. Edit dev-template deployment.yaml to add the secret ref within env config.

2. Now regenerate the dev-overlays and push to argo-cd
```
./infrastructure/scripts/create-dev-overlays.py && argocd app sync hello-world --local ./hello-world/manifests/dev-matt --prune
```

3. Get the correct podname: 
```
kubectl get pods -n hello-world
```

4. Then verify that the secret is now exposed as an env within the pod. One in the pod, type `env` and look for your secret.
```
kubectl exec -n hello-world --stdin --tty <POD_NAME> -- bash
```


## Production Secrets

Make sure `kubectl` points to the production cluster:
```
kubectl config use-context production 
```

Seal the secret:
```
cat favourite-cuisine.yaml | kubeseal --format yaml > favourite-cuisine-sealed.yaml
```

Place the generated Sealed Secret in <app>/manifests/production

Remember to switch back to your own cluster:
   ```
   kubectl config use-context dev-matt
   ```
