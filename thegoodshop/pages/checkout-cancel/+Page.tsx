export default function CheckoutCancelPage() {
  return (
    <div className="p-8 flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-4">Payment Cancelled</h1>
      <p className="text-lg">
        Your payment was cancelled. You can try again or return to the shop.
      </p>
      <a className="btn btn-primary mt-6" href="/">
        Return to Shop
      </a>
    </div>
  );
}
