<script lang="ts">
    import App from "../App.svelte";

	export let data;
</script>

<div class="rounded-lg mb-2 m-4 mx-8 flex bg-slate-300">
	<div>
		<h1 class="font-semibold bg-red-500 rounded-tl-lg p-2 text-white">{ data["title"] }</h1>
		<h1 class="p-2">{ data["description"]}</h1>

	
		<a class="px-2 pt-1 font-semibold" href={data["source"]["url"]}>Soruce: {data["source"]["name"]}</a>
	</div>
	<img class="w-24 rounded-r-lg" src={data["image"]} alt="" />
</div>