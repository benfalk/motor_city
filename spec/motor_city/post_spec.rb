# frozen_string_literal: true

RSpec.describe MotorCity::Post do
  describe "#find" do
    context "with a known PK in the database" do
      subject { described_class.find(1) }

      it { is_expected.to be_a(MotorCity::Post) }
      its([:id]) { is_expected.to eq(1) }
      its([:title]) { is_expected.to eq("Sample Post") }
      its([:body]) { is_expected.to eq("This is a sample post with just a little data") }
      its([:published]) { is_expected.to be(true) }
    end

    context "with a PK not in the database" do
      subject { described_class.find(-1) }
      it { is_expected.to be_nil }
    end
  end

  describe "#all" do
    subject { described_class.all }

    its(:size) { is_expected.to eq(3) }
    it { is_expected.to all(be_a(MotorCity::Post)) }
  end
end
