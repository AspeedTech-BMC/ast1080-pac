#[doc = "Register `OTP_REG160` reader"]
pub type R = crate::R<OtpReg160Spec>;
#[doc = "Register `OTP_REG160` writer"]
pub type W = crate::W<OtpReg160Spec>;
#[doc = "Field `REGREGIONCALIPTRA0REN` reader - REG_REGION_CALIPTRA0_REN"]
pub type Regregioncaliptra0renR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA0REN` writer - REG_REGION_CALIPTRA0_REN"]
pub type Regregioncaliptra0renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA0WEN` reader - REG_REGION_CALIPTRA0_WEN"]
pub type Regregioncaliptra0wenR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA0WEN` writer - REG_REGION_CALIPTRA0_WEN"]
pub type Regregioncaliptra0wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA0LOCK` reader - REG_REGION_CALIPTRA0_LOCK"]
pub type Regregioncaliptra0lockR = crate::BitReader;
#[doc = "Field `REGREGIONCALIPTRA0LOCK` writer - REG_REGION_CALIPTRA0_LOCK"]
pub type Regregioncaliptra0lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA0_REN"]
    #[inline(always)]
    pub fn regregioncaliptra0ren(&self) -> Regregioncaliptra0renR {
        Regregioncaliptra0renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA0_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra0wen(&self) -> Regregioncaliptra0wenR {
        Regregioncaliptra0wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA0_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra0lock(&self) -> Regregioncaliptra0lockR {
        Regregioncaliptra0lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA0_REN"]
    #[inline(always)]
    pub fn regregioncaliptra0ren(&mut self) -> Regregioncaliptra0renW<OtpReg160Spec> {
        Regregioncaliptra0renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA0_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra0wen(&mut self) -> Regregioncaliptra0wenW<OtpReg160Spec> {
        Regregioncaliptra0wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA0_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra0lock(&mut self) -> Regregioncaliptra0lockW<OtpReg160Spec> {
        Regregioncaliptra0lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg160Spec;
impl crate::RegisterSpec for OtpReg160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg160::R`](R) reader structure"]
impl crate::Readable for OtpReg160Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg160::W`](W) writer structure"]
impl crate::Writable for OtpReg160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG160 to value 0xffff"]
impl crate::Resettable for OtpReg160Spec {
    const RESET_VALUE: u32 = 0xffff;
}
