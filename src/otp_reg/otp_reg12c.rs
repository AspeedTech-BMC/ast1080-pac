#[doc = "Register `OTP_REG12C` reader"]
pub type R = crate::R<OtpReg12cSpec>;
#[doc = "Register `OTP_REG12C` writer"]
pub type W = crate::W<OtpReg12cSpec>;
#[doc = "Field `REGREGIONSECURE1STARTOFFSET` reader - REG_REGION_SECURE1_START_OFFSET"]
pub type Regregionsecure1startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE1STARTOFFSET` writer - REG_REGION_SECURE1_START_OFFSET"]
pub type Regregionsecure1startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONSECURE1SIZE` reader - REG_REGION_SECURE1_SIZE"]
pub type Regregionsecure1sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE1SIZE` writer - REG_REGION_SECURE1_SIZE"]
pub type Regregionsecure1sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_SECURE1_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure1startoffset(&self) -> Regregionsecure1startoffsetR {
        Regregionsecure1startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE1_SIZE"]
    #[inline(always)]
    pub fn regregionsecure1size(&self) -> Regregionsecure1sizeR {
        Regregionsecure1sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_SECURE1_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure1startoffset(&mut self) -> Regregionsecure1startoffsetW<OtpReg12cSpec> {
        Regregionsecure1startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE1_SIZE"]
    #[inline(always)]
    pub fn regregionsecure1size(&mut self) -> Regregionsecure1sizeW<OtpReg12cSpec> {
        Regregionsecure1sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg12cSpec;
impl crate::RegisterSpec for OtpReg12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg12c::R`](R) reader structure"]
impl crate::Readable for OtpReg12cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg12c::W`](W) writer structure"]
impl crate::Writable for OtpReg12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG12C to value 0x0090_0000"]
impl crate::Resettable for OtpReg12cSpec {
    const RESET_VALUE: u32 = 0x0090_0000;
}
