#[doc = "Register `OTP_REG170` reader"]
pub type R = crate::R<OtpReg170Spec>;
#[doc = "Register `OTP_REG170` writer"]
pub type W = crate::W<OtpReg170Spec>;
#[doc = "Field `REGREGIONCALIPTRA2REN` reader - REG_REGION_CALIPTRA2_REN"]
pub type Regregioncaliptra2renR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA2REN` writer - REG_REGION_CALIPTRA2_REN"]
pub type Regregioncaliptra2renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA2WEN` reader - REG_REGION_CALIPTRA2_WEN"]
pub type Regregioncaliptra2wenR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA2WEN` writer - REG_REGION_CALIPTRA2_WEN"]
pub type Regregioncaliptra2wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA2LOCK` reader - REG_REGION_CALIPTRA2_LOCK"]
pub type Regregioncaliptra2lockR = crate::BitReader;
#[doc = "Field `REGREGIONCALIPTRA2LOCK` writer - REG_REGION_CALIPTRA2_LOCK"]
pub type Regregioncaliptra2lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA2_REN"]
    #[inline(always)]
    pub fn regregioncaliptra2ren(&self) -> Regregioncaliptra2renR {
        Regregioncaliptra2renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA2_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra2wen(&self) -> Regregioncaliptra2wenR {
        Regregioncaliptra2wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA2_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra2lock(&self) -> Regregioncaliptra2lockR {
        Regregioncaliptra2lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA2_REN"]
    #[inline(always)]
    pub fn regregioncaliptra2ren(&mut self) -> Regregioncaliptra2renW<OtpReg170Spec> {
        Regregioncaliptra2renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA2_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra2wen(&mut self) -> Regregioncaliptra2wenW<OtpReg170Spec> {
        Regregioncaliptra2wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA2_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra2lock(&mut self) -> Regregioncaliptra2lockW<OtpReg170Spec> {
        Regregioncaliptra2lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg170::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg170::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg170Spec;
impl crate::RegisterSpec for OtpReg170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg170::R`](R) reader structure"]
impl crate::Readable for OtpReg170Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg170::W`](W) writer structure"]
impl crate::Writable for OtpReg170Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG170 to value 0xffff"]
impl crate::Resettable for OtpReg170Spec {
    const RESET_VALUE: u32 = 0xffff;
}
