#[doc = "Register `OTP_REG124` reader"]
pub type R = crate::R<OtpReg124Spec>;
#[doc = "Register `OTP_REG124` writer"]
pub type W = crate::W<OtpReg124Spec>;
#[doc = "Field `REGREGIONSECURE0STARTOFFSET` reader - REG_REGION_SECURE0_START_OFFSET"]
pub type Regregionsecure0startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE0STARTOFFSET` writer - REG_REGION_SECURE0_START_OFFSET"]
pub type Regregionsecure0startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONSECURE0SIZE` reader - REG_REGION_SECURE0_SIZE"]
pub type Regregionsecure0sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE0SIZE` writer - REG_REGION_SECURE0_SIZE"]
pub type Regregionsecure0sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_SECURE0_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure0startoffset(&self) -> Regregionsecure0startoffsetR {
        Regregionsecure0startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE0_SIZE"]
    #[inline(always)]
    pub fn regregionsecure0size(&self) -> Regregionsecure0sizeR {
        Regregionsecure0sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_SECURE0_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure0startoffset(&mut self) -> Regregionsecure0startoffsetW<OtpReg124Spec> {
        Regregionsecure0startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE0_SIZE"]
    #[inline(always)]
    pub fn regregionsecure0size(&mut self) -> Regregionsecure0sizeW<OtpReg124Spec> {
        Regregionsecure0sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_SECURE0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg124Spec;
impl crate::RegisterSpec for OtpReg124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg124::R`](R) reader structure"]
impl crate::Readable for OtpReg124Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg124::W`](W) writer structure"]
impl crate::Writable for OtpReg124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG124 to value 0x0090_0000"]
impl crate::Resettable for OtpReg124Spec {
    const RESET_VALUE: u32 = 0x0090_0000;
}
