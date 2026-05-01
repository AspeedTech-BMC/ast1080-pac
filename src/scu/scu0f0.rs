#[doc = "Register `SCU0F0` reader"]
pub type R = crate::R<Scu0f0Spec>;
#[doc = "Register `SCU0F0` writer"]
pub type W = crate::W<Scu0f0Spec>;
#[doc = "Field `SCURNGDIS` reader - SCU_RNG_DIS"]
pub type ScurngdisR = crate::BitReader;
#[doc = "Field `SCURNGDIS` writer - SCU_RNG_DIS"]
pub type ScurngdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURNGMODE` reader - SCU_RNG_MODE"]
pub type ScurngmodeR = crate::FieldReader;
#[doc = "Field `SCURNGMODE` writer - SCU_RNG_MODE"]
pub type ScurngmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCURNGTYPE` reader - SCU_RNG_TYPE"]
pub type ScurngtypeR = crate::BitReader;
#[doc = "Field `SCURNGTYPE` writer - SCU_RNG_TYPE"]
pub type ScurngtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURNGSRC` reader - SCU_RNG_SRC"]
pub type ScurngsrcR = crate::BitReader;
#[doc = "Field `SCURNGSRC` writer - SCU_RNG_SRC"]
pub type ScurngsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURNGVLD` reader - SCU_RNG_VLD"]
pub type ScurngvldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SCU_RNG_DIS"]
    #[inline(always)]
    pub fn scurngdis(&self) -> ScurngdisR {
        ScurngdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SCU_RNG_MODE"]
    #[inline(always)]
    pub fn scurngmode(&self) -> ScurngmodeR {
        ScurngmodeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - SCU_RNG_TYPE"]
    #[inline(always)]
    pub fn scurngtype(&self) -> ScurngtypeR {
        ScurngtypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_RNG_SRC"]
    #[inline(always)]
    pub fn scurngsrc(&self) -> ScurngsrcR {
        ScurngsrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_RNG_VLD"]
    #[inline(always)]
    pub fn scurngvld(&self) -> ScurngvldR {
        ScurngvldR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_RNG_DIS"]
    #[inline(always)]
    pub fn scurngdis(&mut self) -> ScurngdisW<Scu0f0Spec> {
        ScurngdisW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SCU_RNG_MODE"]
    #[inline(always)]
    pub fn scurngmode(&mut self) -> ScurngmodeW<Scu0f0Spec> {
        ScurngmodeW::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_RNG_TYPE"]
    #[inline(always)]
    pub fn scurngtype(&mut self) -> ScurngtypeW<Scu0f0Spec> {
        ScurngtypeW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_RNG_SRC"]
    #[inline(always)]
    pub fn scurngsrc(&mut self) -> ScurngsrcW<Scu0f0Spec> {
        ScurngsrcW::new(self, 5)
    }
}
#[doc = "Random Number Generator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0f0Spec;
impl crate::RegisterSpec for Scu0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0f0::R`](R) reader structure"]
impl crate::Readable for Scu0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0f0::W`](W) writer structure"]
impl crate::Writable for Scu0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0F0 to value 0"]
impl crate::Resettable for Scu0f0Spec {}
