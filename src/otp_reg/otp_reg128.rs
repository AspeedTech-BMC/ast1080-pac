#[doc = "Register `OTP_REG128` reader"]
pub type R = crate::R<OtpReg128Spec>;
#[doc = "Register `OTP_REG128` writer"]
pub type W = crate::W<OtpReg128Spec>;
#[doc = "Field `REGREGIONSECURE1REN` reader - REG_REGION_SECURE1_REN"]
pub type Regregionsecure1renR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE1REN` writer - REG_REGION_SECURE1_REN"]
pub type Regregionsecure1renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE1WEN` reader - REG_REGION_SECURE1_WEN"]
pub type Regregionsecure1wenR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE1WEN` writer - REG_REGION_SECURE1_WEN"]
pub type Regregionsecure1wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE1LOCK` reader - REG_REGION_SECURE1_LOCK"]
pub type Regregionsecure1lockR = crate::BitReader;
#[doc = "Field `REGREGIONSECURE1LOCK` writer - REG_REGION_SECURE1_LOCK"]
pub type Regregionsecure1lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_SECURE1_REN"]
    #[inline(always)]
    pub fn regregionsecure1ren(&self) -> Regregionsecure1renR {
        Regregionsecure1renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE1_WEN"]
    #[inline(always)]
    pub fn regregionsecure1wen(&self) -> Regregionsecure1wenR {
        Regregionsecure1wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE1_LOCK"]
    #[inline(always)]
    pub fn regregionsecure1lock(&self) -> Regregionsecure1lockR {
        Regregionsecure1lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_SECURE1_REN"]
    #[inline(always)]
    pub fn regregionsecure1ren(&mut self) -> Regregionsecure1renW<OtpReg128Spec> {
        Regregionsecure1renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE1_WEN"]
    #[inline(always)]
    pub fn regregionsecure1wen(&mut self) -> Regregionsecure1wenW<OtpReg128Spec> {
        Regregionsecure1wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE1_LOCK"]
    #[inline(always)]
    pub fn regregionsecure1lock(&mut self) -> Regregionsecure1lockW<OtpReg128Spec> {
        Regregionsecure1lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_SECURE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg128Spec;
impl crate::RegisterSpec for OtpReg128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg128::R`](R) reader structure"]
impl crate::Readable for OtpReg128Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg128::W`](W) writer structure"]
impl crate::Writable for OtpReg128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG128 to value 0xffff"]
impl crate::Resettable for OtpReg128Spec {
    const RESET_VALUE: u32 = 0xffff;
}
