#[doc = "Register `OTP_REG31C` reader"]
pub type R = crate::R<OtpReg31cSpec>;
#[doc = "Register `OTP_REG31C` writer"]
pub type W = crate::W<OtpReg31cSpec>;
#[doc = "Field `REGSWINFO7` reader - REG_SW_INFO7"]
pub type Regswinfo7R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO7` writer - REG_SW_INFO7"]
pub type Regswinfo7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO7"]
    #[inline(always)]
    pub fn regswinfo7(&self) -> Regswinfo7R {
        Regswinfo7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO7"]
    #[inline(always)]
    pub fn regswinfo7(&mut self) -> Regswinfo7W<OtpReg31cSpec> {
        Regswinfo7W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE7\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg31c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg31c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg31cSpec;
impl crate::RegisterSpec for OtpReg31cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg31c::R`](R) reader structure"]
impl crate::Readable for OtpReg31cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg31c::W`](W) writer structure"]
impl crate::Writable for OtpReg31cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG31C to value 0x07"]
impl crate::Resettable for OtpReg31cSpec {
    const RESET_VALUE: u32 = 0x07;
}
