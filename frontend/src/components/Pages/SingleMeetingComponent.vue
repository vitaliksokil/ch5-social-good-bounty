<template>
  <div class="site-section">
    <div class="container">

      <div class="row mb-5 align-items-st">
        <div class="col-md-5">
          <div class="heading-20219">
            <h2 class="title text-cursive mb-4">{{ currentMeeting.title }}</h2>
            <p>{{currentMeeting.description}}</p>
            <p><button class="btn btn-primary rounded-0 px-4" @click.prevent="buyTicketAction" :disabled="currentMeeting.bought_tickets == currentMeeting.total_tickets">Buy Ticket</button></p>
          </div>
        </div>
        <div class="col-md-7">
          <div class="cause shadow-sm">

            <a class="cause-link d-block">
              <img :src="currentMeeting.nft_media" alt="Image" class="img-fluid">
              <custom-progress-bar :current-value="currentMeeting.bought_tickets" :max-value="currentMeeting.total_tickets"></custom-progress-bar>
            </a>

            <div class="px-3 pt-3 border-top-0 border border shadow-sm">
              <span class="py-1 small px-2 rounded mb-3 d-inline-block" :class="currentDonationTypeClassName">
                <font-awesome-icon :icon="currentDonationTypeIconName" /> {{ currentMeeting.donation_type }}</span>
              <h3 class="mb-4">{{ currentMeeting.title }}</h3>
              <div class="border-top border-light border-bottom py-2 d-flex justify-content-between">
                <div>Ticket price</div>
                <div class="ml-auto"><strong class="text-success">{{ $filters.formatTicketPrice(currentMeeting.ticket_price) }} Ⓝ</strong></div>
              </div>

              <div class="py-4">
                <div class="d-flex align-items-center">
                  <div class="">{{ currentMeeting.owner_name }}</div>
                </div>
              </div>
            </div>

          </div>
        </div>
      </div>
    </div>
  </div>


</template>

<script>
import CustomProgressBar from "@/components/Items/CustomProgressBar";
import Big from "big.js";
const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

export default {
  name: "SingleMeetingComponent",
  components: {
    CustomProgressBar,
  },
  data(){
    return {
      currentMeeting:{},
      currentDonationTypeClassName:'',
      currentDonationTypeIconName:'',
    }
  },
  methods:{
    async getCurrentMeeting(){
      const argsBase64 = window.btoa(JSON.stringify({id:+this.$route.params.id}))
      let result = await window.provider.query({
        request_type: "call_function",
        account_id: window.walletSelector.getContractId(),
        method_name: "get_meeting_by_id",
        args_base64: argsBase64,
        finality: "optimistic",
      }).catch(()=>{
        this.$router.push({'name':'home'});
      })
      this.currentMeeting = JSON.parse(Buffer.from(result.result).toString())
    },

    async buyTicket() {
      await window.walletSelector.signAndSendTransaction({
        signerId: window.nearAccount.accountId,
        // receiverId: nearConfig.contractName,
        actions:[
          {
            type: "FunctionCall",
            params: {
              methodName: "buy_ticket",
              args: {
                "meeting_id": +this.$route.params.id,
              },
              gas: BOATLOAD_OF_GAS,
              deposit: Big((this.currentMeeting.ticket_price / (10**24)).toFixed(5)).times(10 ** 24).toFixed()
            },
          },
        ]
      });
    },

    async buyTicketAction(){
      let loader = this.$loading.show();
      try {
        await this.buyTicket();
        this.$swal.fire({
          icon: 'success',
          title: 'Success',
          text: 'Ticket was successfully bought!',
          footer: `To look at your ticket go to your wallet collectibles!`,
        })
        await this.getCurrentMeeting();
      }catch (error){
        this.$swal.fire({
          icon: 'error',
          title: 'Error',
          text: error.message,
          footer: ``,
        })
        console.log(error)
      }
      loader.hide();

    }
  },
  async mounted() {
    let loader = this.$loading.show();
    await this.getCurrentMeeting();
    this.currentDonationTypeClassName =this.$DONATION_TYPES[this.currentMeeting.donation_type].className;
    this.currentDonationTypeIconName =  this.$DONATION_TYPES[this.currentMeeting.donation_type].iconName;
    loader.hide();

    let uri = window.location.search.substring(1);
    let params = new URLSearchParams(uri);
    const transactionHash = params.get('transactionHashes');
    if(transactionHash){
      this.$swal.fire({
        icon: 'success',
        title: 'Success',
        text: 'Ticket was successfully bought!',
        footer: `To look at your ticket go to your wallet collectibles!`,
      })
    }
  },
}
</script>

<style scoped>

</style>