const filters = {
    formatTicketPrice(price) {
        return (price / (10 ** 24)).toFixed(5)
    }
}
export default filters;
