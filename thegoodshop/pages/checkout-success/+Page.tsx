export default function CheckoutSuccessPage() {
  return (
    <div className="p-8 flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-4">Payment Successful!</h1>
      <p className="text-lg">
        Thank you for your purchase. Your payment has been processed successfully.
      </p>
      <a className="btn btn-primary mt-6" href="/">
        Return to Shop
      </a>
    </div>
  );
}
