#[doc = "Register `OTP_REG324` reader"]
pub type R = crate::R<OtpReg324Spec>;
#[doc = "Register `OTP_REG324` writer"]
pub type W = crate::W<OtpReg324Spec>;
#[doc = "Field `REGSWINFO9` reader - REG_SW_INFO9"]
pub type Regswinfo9R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO9` writer - REG_SW_INFO9"]
pub type Regswinfo9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO9"]
    #[inline(always)]
    pub fn regswinfo9(&self) -> Regswinfo9R {
        Regswinfo9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO9"]
    #[inline(always)]
    pub fn regswinfo9(&mut self) -> Regswinfo9W<OtpReg324Spec> {
        Regswinfo9W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE9\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg324::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg324::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg324Spec;
impl crate::RegisterSpec for OtpReg324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg324::R`](R) reader structure"]
impl crate::Readable for OtpReg324Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg324::W`](W) writer structure"]
impl crate::Writable for OtpReg324Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG324 to value 0x09"]
impl crate::Resettable for OtpReg324Spec {
    const RESET_VALUE: u32 = 0x09;
}
