<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte"

</script>

<div class="flex flex-col justify-center items-center h-11/12 bg-background">
  <div class="text-center">
    <div class="flex items-center justify-center pb-12">
      <img src="/icons/lock.png" alt="Lock icon" class="size-24" />

    </div>
    <h1 class="font-bold text-3xl pb-3">Welcome to TotLaUnLoc</h1>
    <p class="">A really smart password manager and helper</p>
    
    <Button variant="outline" class="mt-12 w-1/2">
      <a href="/passwords">Get started</a>
    </Button>
  </div>
</div>