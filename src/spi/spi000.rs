#[doc = "Register `SPI000` reader"]
pub type R = crate::R<Spi000Spec>;
#[doc = "Register `SPI000` writer"]
pub type W = crate::W<Spi000Spec>;
#[doc = "Field `CE0FLASHTYPE` reader - CE0_FLASH_TYPE"]
pub type Ce0flashtypeR = crate::FieldReader;
#[doc = "Field `CE1FLASHTYPE` reader - CE1_FLASH_TYPE"]
pub type Ce1flashtypeR = crate::FieldReader;
#[doc = "Field `CE2FLASHTYPE` reader - CE2_FLASH_TYPE"]
pub type Ce2flashtypeR = crate::FieldReader;
#[doc = "Field `CE3FLASHTYPE` reader - CE3_FLASH_TYPE"]
pub type Ce3flashtypeR = crate::FieldReader;
#[doc = "Field `CE0WE` reader - CE0_WE"]
pub type Ce0weR = crate::BitReader;
#[doc = "Field `CE0WE` writer - CE0_WE"]
pub type Ce0weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE1WE` reader - CE1_WE"]
pub type Ce1weR = crate::BitReader;
#[doc = "Field `CE1WE` writer - CE1_WE"]
pub type Ce1weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE2WE` reader - CE2_WE"]
pub type Ce2weR = crate::BitReader;
#[doc = "Field `CE2WE` writer - CE2_WE"]
pub type Ce2weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE3WE` reader - CE3_WE"]
pub type Ce3weR = crate::BitReader;
#[doc = "Field `CE3WE` writer - CE3_WE"]
pub type Ce3weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAPVAL` reader - TRAP_VAL"]
pub type TrapvalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - CE0_FLASH_TYPE"]
    #[inline(always)]
    pub fn ce0flashtype(&self) -> Ce0flashtypeR {
        Ce0flashtypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CE1_FLASH_TYPE"]
    #[inline(always)]
    pub fn ce1flashtype(&self) -> Ce1flashtypeR {
        Ce1flashtypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CE2_FLASH_TYPE"]
    #[inline(always)]
    pub fn ce2flashtype(&self) -> Ce2flashtypeR {
        Ce2flashtypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CE3_FLASH_TYPE"]
    #[inline(always)]
    pub fn ce3flashtype(&self) -> Ce3flashtypeR {
        Ce3flashtypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - CE0_WE"]
    #[inline(always)]
    pub fn ce0we(&self) -> Ce0weR {
        Ce0weR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CE1_WE"]
    #[inline(always)]
    pub fn ce1we(&self) -> Ce1weR {
        Ce1weR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CE2_WE"]
    #[inline(always)]
    pub fn ce2we(&self) -> Ce2weR {
        Ce2weR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CE3_WE"]
    #[inline(always)]
    pub fn ce3we(&self) -> Ce3weR {
        Ce3weR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - TRAP_VAL"]
    #[inline(always)]
    pub fn trapval(&self) -> TrapvalR {
        TrapvalR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - CE0_WE"]
    #[inline(always)]
    pub fn ce0we(&mut self) -> Ce0weW<Spi000Spec> {
        Ce0weW::new(self, 16)
    }
    #[doc = "Bit 17 - CE1_WE"]
    #[inline(always)]
    pub fn ce1we(&mut self) -> Ce1weW<Spi000Spec> {
        Ce1weW::new(self, 17)
    }
    #[doc = "Bit 18 - CE2_WE"]
    #[inline(always)]
    pub fn ce2we(&mut self) -> Ce2weW<Spi000Spec> {
        Ce2weW::new(self, 18)
    }
    #[doc = "Bit 19 - CE3_WE"]
    #[inline(always)]
    pub fn ce3we(&mut self) -> Ce3weW<Spi000Spec> {
        Ce3weW::new(self, 19)
    }
}
#[doc = "SPI Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi000Spec;
impl crate::RegisterSpec for Spi000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi000::R`](R) reader structure"]
impl crate::Readable for Spi000Spec {}
#[doc = "`write(|w| ..)` method takes [`spi000::W`](W) writer structure"]
impl crate::Writable for Spi000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI000 to value 0xaa"]
impl crate::Resettable for Spi000Spec {
    const RESET_VALUE: u32 = 0xaa;
}
