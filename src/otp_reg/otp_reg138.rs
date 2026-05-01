#[doc = "Register `OTP_REG138` reader"]
pub type R = crate::R<OtpReg138Spec>;
#[doc = "Register `OTP_REG138` writer"]
pub type W = crate::W<OtpReg138Spec>;
#[doc = "Field `REGREGIONSECURE3REN` reader - REG_REGION_SECURE3_REN"]
pub type Regregionsecure3renR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE3REN` writer - REG_REGION_SECURE3_REN"]
pub type Regregionsecure3renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE3WEN` reader - REG_REGION_SECURE3_WEN"]
pub type Regregionsecure3wenR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE3WEN` writer - REG_REGION_SECURE3_WEN"]
pub type Regregionsecure3wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE3LOCK` reader - REG_REGION_SECURE3_LOCK"]
pub type Regregionsecure3lockR = crate::BitReader;
#[doc = "Field `REGREGIONSECURE3LOCK` writer - REG_REGION_SECURE3_LOCK"]
pub type Regregionsecure3lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_SECURE3_REN"]
    #[inline(always)]
    pub fn regregionsecure3ren(&self) -> Regregionsecure3renR {
        Regregionsecure3renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE3_WEN"]
    #[inline(always)]
    pub fn regregionsecure3wen(&self) -> Regregionsecure3wenR {
        Regregionsecure3wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE3_LOCK"]
    #[inline(always)]
    pub fn regregionsecure3lock(&self) -> Regregionsecure3lockR {
        Regregionsecure3lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_SECURE3_REN"]
    #[inline(always)]
    pub fn regregionsecure3ren(&mut self) -> Regregionsecure3renW<OtpReg138Spec> {
        Regregionsecure3renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE3_WEN"]
    #[inline(always)]
    pub fn regregionsecure3wen(&mut self) -> Regregionsecure3wenW<OtpReg138Spec> {
        Regregionsecure3wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE3_LOCK"]
    #[inline(always)]
    pub fn regregionsecure3lock(&mut self) -> Regregionsecure3lockW<OtpReg138Spec> {
        Regregionsecure3lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_SECURE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg138Spec;
impl crate::RegisterSpec for OtpReg138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg138::R`](R) reader structure"]
impl crate::Readable for OtpReg138Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg138::W`](W) writer structure"]
impl crate::Writable for OtpReg138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG138 to value 0xffff"]
impl crate::Resettable for OtpReg138Spec {
    const RESET_VALUE: u32 = 0xffff;
}
