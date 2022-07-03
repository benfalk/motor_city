# frozen_string_literal: true

RSpec.describe MotorCity::FFI do
  describe "#db_url" do
    subject { described_class.db_url }
    it { is_expected.to eq(ENV.fetch("DATABASE_URL", "")) }
  end

  describe "#connection_ok" do
    subject { described_class.connection_ok(MotorCity.connection) }
    it { is_expected.to be(true) }
  end
end
