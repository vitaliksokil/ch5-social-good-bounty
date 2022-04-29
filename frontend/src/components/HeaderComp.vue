<template>
  <header id="header" class="d-flex align-items-center">
    <div class="container d-flex align-items-center">

      <h1 class="logo me-auto">
        <router-link :to="{name:'home'}">{{appName}}</router-link>
      </h1>
      <!-- Uncomment below if you prefer to use an image logo -->
      <!-- <a href="index.html" class="logo me-auto"><img src="assets/img/logo.png" alt="" class="img-fluid"></a>-->

      <nav id="navbar" class="navbar">
        <ul>
          <li>
            <router-link
                :to="{name:'home'}"
                custom
                v-slot="{ href, navigate, isExactActive }">
              <a :href="href" @click="navigate"
                 class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >Home</a>
            </router-link>
          </li>
          <li>
            <router-link
                :to="{name:'about'}"
                custom
                v-slot="{ href, navigate, isExactActive }">
              <a :href="href" @click="navigate"
                 class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >About</a>
            </router-link>
          </li>
          <li>
            <router-link
                :to="{name:'meetings'}"
                custom
                v-slot="{ href, navigate, isExactActive }">
              <a :href="href" @click="navigate"
                 class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >Meetings</a>
            </router-link>
          </li>
          <li class="dropdown" style="margin-left: 30px"  v-if="isSignedIn">
            <a href="#" class="text-decoration-underline"><span class="mx-2">{{ shortAddressId }}</span> <font-awesome-icon icon="caret-down" /></a>
            <ul class="p-2">
              <li><button class="btn btn-outline-success w-100" v-clipboard:copy="accountId" >{{ shortAddressId }} <font-awesome-icon icon="copy" /></button></li>
              <hr>
              <li>
                <router-link
                    :to="{name:'profile-dashboard'}"
                    custom
                    v-slot="{ href, navigate, isExactActive }">
                  <a :href="href" @click="navigate"
                     class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >Profile</a>
                </router-link>
              </li>
<!--              <li><a href="#">Drop Down 3</a></li>-->
              <hr>
              <li v-if="isSignedIn"><button class="btn btn-outline-danger w-100"  @click.prevent="signOut" >Sign Out</button></li>
            </ul>
          </li>
          <li style="margin-left: 30px" v-else><button class="btn btn-outline-success m-2" @click.prevent="login" >Log In</button></li>
        </ul>
        <i class="bi bi-list mobile-nav-toggle"></i>
      </nav><!-- .navbar -->

    </div>
  </header>
</template>

<script>
import {signIn, signOut} from "@/utils";

export default {
  name: "HeaderComp",
  props:["accountId", "isSignedIn"],
  filters: {
    shortAddress: function (value) {
      if (!value) return ''
      if(value.length > 25){
        return value.substring(0, 5) + '...' + value.substring(value.length - 5);
      }
      return value;
    }
  },
  methods:{
    async login(){
      if(!this.isSignedIn){
        await signIn();
      }
    },
    async signOut(){
      await signOut();
      this.$parent.updateInfo();
    },
  },
  computed:{
    appName(){
      return process.env.VUE_APP_APP_NAME
    },
    shortAddressId(){
      return this.$options.filters.shortAddress(this.accountId)
    }
  }
}
</script>

<style scoped>


/*--------------------------------------------------------------
# Header
--------------------------------------------------------------*/
#header {
  background: #fff;
  transition: all 0.5s;
  z-index: 997;
  height: 70px;
}
#header.header-scrolled {
  box-shadow: 0px 2px 15px rgba(0, 0, 0, 0.1);
}
#header .logo {
  font-size: 30px;
  margin: 0;
  padding: 0;
  line-height: 1;
  font-weight: 700;
  letter-spacing: 1px;
  text-transform: uppercase;
}
#header .logo a {
  color: #5cb874;
}
#header .logo img {
  max-height: 40px;
}

.scrolled-offset {
  margin-top: 70px;
}


/*--------------------------------------------------------------
# Navigation Menu
--------------------------------------------------------------*/
/**
* Desktop Navigation
*/
.navbar {
  padding: 0;
}
.navbar ul {
  margin: 0;
  padding: 0;
  display: flex;
  list-style: none;
  align-items: center;
}
.navbar li {
  position: relative;
}
.navbar a, .navbar a:focus {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 0 10px 30px;
  font-size: 15px;
  color: #222222;
  white-space: nowrap;
  transition: 0.3s;
}
.navbar a i, .navbar a:focus i {
  font-size: 12px;
  line-height: 0;
  margin-left: 5px;
}
.navbar a:hover, .navbar .active, .navbar .active:focus, .navbar li:hover > a {
  color: #5cb874;
}
.navbar .getstarted, .navbar .getstarted:focus {
  color: #5cb874;
  padding: 8px 25px;
  margin-left: 30px;
  border-radius: 4px;
  border: 2px solid #5cb874;
  transition: 0.3s;
  font-size: 14px;
}
.navbar .getstarted:hover, .navbar .getstarted:focus:hover {
  background: #5cb874;
  color: #fff;
}
.navbar .dropdown ul {
  display: block;
  position: absolute;
  left: 14px;
  top: calc(100% + 30px);
  margin: 0;
  padding: 10px 0;
  z-index: 99;
  opacity: 0;
  visibility: hidden;
  background: #fff;
  box-shadow: 0px 0px 30px rgba(127, 137, 161, 0.25);
  transition: 0.3s;
}
.navbar .dropdown ul li {
  min-width: 200px;
}
.navbar .dropdown ul a {
  padding: 10px 20px;
  text-transform: none;
}
.navbar .dropdown ul a i {
  font-size: 12px;
}
.navbar .dropdown ul a:hover, .navbar .dropdown ul .active:hover, .navbar .dropdown ul li:hover > a {
  color: #5cb874;
}
.navbar .dropdown:hover > ul {
  opacity: 1;
  top: 100%;
  visibility: visible;
}
.navbar .dropdown .dropdown ul {
  top: 0;
  left: calc(100% - 30px);
  visibility: hidden;
}
.navbar .dropdown .dropdown:hover > ul {
  opacity: 1;
  top: 0;
  left: 100%;
  visibility: visible;
}
@media (max-width: 1366px) {
  .navbar .dropdown .dropdown ul {
    left: -90%;
  }
  .navbar .dropdown .dropdown:hover > ul {
    left: -100%;
  }
}
</style>