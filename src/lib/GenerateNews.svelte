<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import Article from "./Article.svelte";
    import Greet from "./Greet.svelte";
	
	let articles: any[] = [];
	
	async function generateNews() {
		articles = await invoke("get_latest_news");
		console.log(articles)
	}
</script>

<div class="flex flex-col">
	<div class="justify-center flex w-full">
		<button class="bg-red-500 rounded-lg p-2 mb-2 text-white font-bold hover:bg-red-400 transition ease-in" on:click={generateNews}>Get News!</button>
	</div>

	<div class="flex flex-col">
		{#each articles as article}
			<Article data={article}></Article>
		{/each}
	</div>
</div>
