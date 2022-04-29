<template>
  <!-- ======= Hero Section ======= -->
  <section id="hero">
    <div id="heroCarousel" data-bs-interval="5000" class="carousel slide carousel-fade" data-bs-ride="carousel">

      <ol class="carousel-indicators" id="hero-carousel-indicators"></ol>

      <div class="carousel-inner" role="listbox">
        <div class="carousel-item active" :style="{ backgroundImage: 'url(' + require('@/assets/images/heroimage.jpg') + ')' }">
          <div class="carousel-container">
            <div class="container">
              <h2 class="animate__animated animate__fadeInDown">Welcome to <span>{{ appName }}</span></h2>
              <p class="animate__animated animate__fadeInUp">Welcome to {{ appName }} project. Here you can buy NFT ticket for meeting with some person. All collected money will be transfer for good cause selected by a creator.</p>
              <router-link
                  :to="{name:'about'}" class="btn-get-started animate__animated animate__fadeInUp scrollto" >
                Read More
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section><!-- End Hero -->

</template>

<script>
export default {
  name: "HomeComponent",
  computed:{
    appName(){
      return process.env.VUE_APP_APP_NAME
    }
  }
}
</script>

<style scoped>

/*--------------------------------------------------------------
# Hero Section
--------------------------------------------------------------*/
#hero {
  width: 100%;
  height: calc(100vh - 70px);
  background-color: rgba(9, 9, 9, 0.8);
  overflow: hidden;
  position: relative;
}
#hero .carousel, #hero .carousel-inner, #hero .carousel-item, #hero .carousel-item::before {
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  bottom: 0;
}
#hero .carousel-item::before {
  content: "";
  background-color: rgba(45, 103, 60, 0.8);
}
#hero .carousel-container {
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  bottom: 0;
  top: 0;
  left: 50px;
  right: 50px;
}
#hero .container {
  text-align: center;
}
#hero h2 {
  color: #fff;
  margin-bottom: 20px;
  font-size: 48px;
  font-weight: 700;
}
#hero p {
  -webkit-animation-delay: 0.4s;
  animation-delay: 0.4s;
  margin: 0 auto 30px auto;
  color: #fff;
}
#hero .carousel-inner .carousel-item {
  transition-property: opacity;
  background-position: center top;
}
#hero .carousel-inner .carousel-item,
#hero .carousel-inner .active.carousel-item-start,
#hero .carousel-inner .active.carousel-item-end {
  opacity: 0;
}
#hero .carousel-inner .active,
#hero .carousel-inner .carousel-item-next.carousel-item-start,
#hero .carousel-inner .carousel-item-prev.carousel-item-end {
  opacity: 1;
  transition: 0.5s;
}
#hero .carousel-control-next-icon, #hero .carousel-control-prev-icon {
  background: none;
  font-size: 30px;
  line-height: 0;
  width: auto;
  height: auto;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 50px;
  transition: 0.3s;
  color: rgba(255, 255, 255, 0.5);
  width: 54px;
  height: 54px;
  display: flex;
  align-items: center;
  justify-content: center;
}
#hero .carousel-control-next-icon:hover, #hero .carousel-control-prev-icon:hover {
  background: rgba(255, 255, 255, 0.3);
  color: rgba(255, 255, 255, 0.8);
}
#hero .carousel-indicators li {
  cursor: pointer;
  background: #fff;
  overflow: hidden;
  border: 0;
  width: 12px;
  height: 12px;
  border-radius: 50px;
  opacity: 0.6;
  transition: 0.3s;
}
#hero .carousel-indicators li.active {
  opacity: 1;
  background: #5cb874;
}
#hero .btn-get-started {
  font-family: "Raleway", sans-serif;
  font-weight: 500;
  font-size: 14px;
  letter-spacing: 1px;
  display: inline-block;
  padding: 14px 32px;
  border-radius: 4px;
  transition: 0.5s;
  line-height: 1;
  color: #fff;
  -webkit-animation-delay: 0.8s;
  animation-delay: 0.8s;
  background: #5cb874;
}
#hero .btn-get-started:hover {
  background: #6ec083;
}
@media (max-width: 992px) {
  #hero {
    height: 100vh;
  }
  #hero .carousel-container {
    top: 66px;
  }
}
@media (max-width: 768px) {
  #hero h2 {
    font-size: 28px;
  }
}
@media (min-width: 1024px) {
  #hero .carousel-control-prev, #hero .carousel-control-next {
    width: 5%;
  }
}
@media (max-height: 500px) {
  #hero {
    height: 120vh;
  }
}
</style>