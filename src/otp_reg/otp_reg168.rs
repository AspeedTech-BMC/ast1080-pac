#[doc = "Register `OTP_REG168` reader"]
pub type R = crate::R<OtpReg168Spec>;
#[doc = "Register `OTP_REG168` writer"]
pub type W = crate::W<OtpReg168Spec>;
#[doc = "Field `REGREGIONCALIPTRA1REN` reader - REG_REGION_CALIPTRA1_REN"]
pub type Regregioncaliptra1renR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA1REN` writer - REG_REGION_CALIPTRA1_REN"]
pub type Regregioncaliptra1renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA1WEN` reader - REG_REGION_CALIPTRA1_WEN"]
pub type Regregioncaliptra1wenR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA1WEN` writer - REG_REGION_CALIPTRA1_WEN"]
pub type Regregioncaliptra1wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA1LOCK` reader - REG_REGION_CALIPTRA1_LOCK"]
pub type Regregioncaliptra1lockR = crate::BitReader;
#[doc = "Field `REGREGIONCALIPTRA1LOCK` writer - REG_REGION_CALIPTRA1_LOCK"]
pub type Regregioncaliptra1lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA1_REN"]
    #[inline(always)]
    pub fn regregioncaliptra1ren(&self) -> Regregioncaliptra1renR {
        Regregioncaliptra1renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA1_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra1wen(&self) -> Regregioncaliptra1wenR {
        Regregioncaliptra1wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA1_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra1lock(&self) -> Regregioncaliptra1lockR {
        Regregioncaliptra1lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA1_REN"]
    #[inline(always)]
    pub fn regregioncaliptra1ren(&mut self) -> Regregioncaliptra1renW<OtpReg168Spec> {
        Regregioncaliptra1renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA1_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra1wen(&mut self) -> Regregioncaliptra1wenW<OtpReg168Spec> {
        Regregioncaliptra1wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA1_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra1lock(&mut self) -> Regregioncaliptra1lockW<OtpReg168Spec> {
        Regregioncaliptra1lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg168Spec;
impl crate::RegisterSpec for OtpReg168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg168::R`](R) reader structure"]
impl crate::Readable for OtpReg168Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg168::W`](W) writer structure"]
impl crate::Writable for OtpReg168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG168 to value 0xffff"]
impl crate::Resettable for OtpReg168Spec {
    const RESET_VALUE: u32 = 0xffff;
}
