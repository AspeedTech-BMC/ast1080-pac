#[doc = "Register `OTP_REG304` reader"]
pub type R = crate::R<OtpReg304Spec>;
#[doc = "Register `OTP_REG304` writer"]
pub type W = crate::W<OtpReg304Spec>;
#[doc = "Field `REGSWINFO1` reader - REG_SW_INFO1"]
pub type Regswinfo1R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO1` writer - REG_SW_INFO1"]
pub type Regswinfo1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO1"]
    #[inline(always)]
    pub fn regswinfo1(&self) -> Regswinfo1R {
        Regswinfo1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO1"]
    #[inline(always)]
    pub fn regswinfo1(&mut self) -> Regswinfo1W<OtpReg304Spec> {
        Regswinfo1W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg304::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg304::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg304Spec;
impl crate::RegisterSpec for OtpReg304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg304::R`](R) reader structure"]
impl crate::Readable for OtpReg304Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg304::W`](W) writer structure"]
impl crate::Writable for OtpReg304Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG304 to value 0x01"]
impl crate::Resettable for OtpReg304Spec {
    const RESET_VALUE: u32 = 0x01;
}
