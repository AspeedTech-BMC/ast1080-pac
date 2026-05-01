#[doc = "Register `OTP_REG0BC` reader"]
pub type R = crate::R<OtpReg0bcSpec>;
#[doc = "Register `OTP_REG0BC` writer"]
pub type W = crate::W<OtpReg0bcSpec>;
#[doc = "Field `REGOTPADDRM5` reader - REG_OTP_ADDR_M5"]
pub type Regotpaddrm5R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM5` writer - REG_OTP_ADDR_M5"]
pub type Regotpaddrm5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M5"]
    #[inline(always)]
    pub fn regotpaddrm5(&self) -> Regotpaddrm5R {
        Regotpaddrm5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M5"]
    #[inline(always)]
    pub fn regotpaddrm5(&mut self) -> Regotpaddrm5W<OtpReg0bcSpec> {
        Regotpaddrm5W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0bcSpec;
impl crate::RegisterSpec for OtpReg0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0bc::R`](R) reader structure"]
impl crate::Readable for OtpReg0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0bc::W`](W) writer structure"]
impl crate::Writable for OtpReg0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0BC to value 0"]
impl crate::Resettable for OtpReg0bcSpec {}
