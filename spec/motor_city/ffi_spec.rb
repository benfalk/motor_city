# frozen_string_literal: true

RSpec.describe MotorCity::FFI do
  describe "#add" do
    let(:left) { 2 }
    let(:right) { 3 }
    subject { described_class.add(left, right) }
    it { is_expected.to eq(5) }
  end

  describe "#db_url" do
    subject { described_class.db_url }
    it { is_expected.to eq(ENV.fetch("DB_URL", "")) }
  end
end
