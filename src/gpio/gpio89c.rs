#[doc = "Register `GPIO89C` reader"]
pub type R = crate::R<Gpio89cSpec>;
#[doc = "Register `GPIO89C` writer"]
pub type W = crate::W<Gpio89cSpec>;
#[doc = "Field `GPIO140WrPrivilegeOfMaster` reader - GPIO140 Write Privilege of Master"]
pub type Gpio140wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO140WrPrivilegeOfMaster` writer - GPIO140 Write Privilege of Master"]
pub type Gpio140wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO141WrPrivilegeOfMaster` reader - GPIO141 Write Privilege of Master"]
pub type Gpio141wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO141WrPrivilegeOfMaster` writer - GPIO141 Write Privilege of Master"]
pub type Gpio141wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO142WrPrivilegeOfMaster` reader - GPIO142 Write Privilege of Master"]
pub type Gpio142wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO142WrPrivilegeOfMaster` writer - GPIO142 Write Privilege of Master"]
pub type Gpio142wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO143WrPrivilegeOfMaster` reader - GPIO143 Write Privilege of Master"]
pub type Gpio143wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO143WrPrivilegeOfMaster` writer - GPIO143 Write Privilege of Master"]
pub type Gpio143wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO140 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio140wr_privilege_of_master(&self) -> Gpio140wrPrivilegeOfMasterR {
        Gpio140wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO141 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio141wr_privilege_of_master(&self) -> Gpio141wrPrivilegeOfMasterR {
        Gpio141wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO142 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio142wr_privilege_of_master(&self) -> Gpio142wrPrivilegeOfMasterR {
        Gpio142wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO143 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio143wr_privilege_of_master(&self) -> Gpio143wrPrivilegeOfMasterR {
        Gpio143wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO140 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio140wr_privilege_of_master(&mut self) -> Gpio140wrPrivilegeOfMasterW<Gpio89cSpec> {
        Gpio140wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO141 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio141wr_privilege_of_master(&mut self) -> Gpio141wrPrivilegeOfMasterW<Gpio89cSpec> {
        Gpio141wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO142 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio142wr_privilege_of_master(&mut self) -> Gpio142wrPrivilegeOfMasterW<Gpio89cSpec> {
        Gpio142wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO143 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio143wr_privilege_of_master(&mut self) -> Gpio143wrPrivilegeOfMasterW<Gpio89cSpec> {
        Gpio143wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio89c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio89c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio89cSpec;
impl crate::RegisterSpec for Gpio89cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio89c::R`](R) reader structure"]
impl crate::Readable for Gpio89cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio89c::W`](W) writer structure"]
impl crate::Writable for Gpio89cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO89C to value 0xffff_ffff"]
impl crate::Resettable for Gpio89cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
