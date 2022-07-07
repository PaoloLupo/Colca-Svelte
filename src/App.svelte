<script lang="ts">
  import Header from "./components/Header.svelte";
  import UserRegistration from "./components/UserRegistration.svelte";
  import Footer from "./components/Footer.svelte";
  import Content from "./Content.svelte";
  import CustomMenu from "./components/CustomMenu/CustomMenu.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import ConfigProps from "./components/ConfigProps.svelte";
</script>

<main>
  <UserRegistration />
  <Header />
  <Sidebar />
  <Content />
  <ConfigProps />
  <Footer />
  <CustomMenu />
</main>
